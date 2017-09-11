use super::errors::*;
use super::function::Function;
use super::op::Op;
use super::rspirv;
use super::spirv::{ExecutionModel, StorageClass, Word};
use super::spirv_type::SpirvType;
use super::type_key::TypeKey;
use super::types::{Float, UnsignedInteger};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub enum ShaderKind {
    Vertex,
    Fragment,
}

impl ShaderKind {
    pub fn as_execution_model(self) -> ExecutionModel {
        use self::ShaderKind::*;

        match self {
            Vertex => ExecutionModel::Vertex,
            Fragment => ExecutionModel::Fragment,
        }
    }
}

pub struct Shader {
    /// Internal builder
    pub(crate) builder: rspirv::mr::Builder,
    /// Cached types, to only initialize each type once.
    type_cache: HashMap<TypeKey, Word>,
    #[cfg(feature = "vulkan")]
    vulkan_shader_interfaces: Vec<self::vulkan::ShaderInterface>,
}

impl fmt::Debug for Shader {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Shader {{ }}")
    }
}


impl Shader {
    pub fn new() -> Shader {
        use super::spirv::Capability;
        use super::spirv::AddressingModel;
        use super::spirv::MemoryModel;

        let mut builder = rspirv::mr::Builder::new();

        builder.capability(Capability::Shader);
        builder.ext_inst_import(String::from("GLSL.std.450"));
        builder.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);

        Shader {
            builder: builder,
            type_cache: HashMap::new(),
            #[cfg(feature = "vulkan")]
            vulkan_shader_interfaces: Vec::new(),
        }
    }

    pub(crate) fn cached_type<I>(&mut self, ty: TypeKey, inserter: I) -> Result<Word>
    where
        I: Fn(&mut Self) -> Result<Word>,
    {
        if let Some(id) = self.type_cache.get(&ty) {
            return Ok(*id);
        }

        let id = inserter(self)?;
        self.type_cache.insert(ty, id);
        Ok(id)
    }

    pub(crate) fn constant_u32(&mut self, value: u32) -> Result<Word> {
        let integer_type = UnsignedInteger.register_type(self)?;

        self.cached_type(
            TypeKey::ConstantU32 {
                integer_type: integer_type,
                value: value,
            },
            |s| Ok(s.builder.constant_u32(integer_type, value)),
        )
    }

    pub(crate) fn constant_f32(&mut self, value: f32) -> Result<Word> {
        let float_type = Float.register_type(self)?;

        self.cached_type(
            TypeKey::ConstantF32 {
                float_type: float_type,
                value: value.to_bits(),
            },
            |s| Ok(s.builder.constant_f32(float_type, value)),
        )
    }

    pub(crate) fn register_pointer_type(
        &mut self,
        storage_class: StorageClass,
        pointee_type: Word,
    ) -> Result<Word> {
        self.cached_type(
            TypeKey::Pointer {
                storage_class: storage_class,
                pointee_type: pointee_type,
            },
            |s| {
                Ok(s.builder.type_pointer(
                    None,
                    storage_class.into(),
                    pointee_type,
                ))
            },
        )
    }

    pub(crate) fn member_name(&mut self, id: Word, index: u32, name: &str) {
        self.builder.member_name(id, index, name.to_string());
    }

    pub(crate) fn name(&mut self, id: Word, name: &str) {
        self.builder.name(id, name.to_string());
    }

    pub fn entry_point(
        &mut self,
        kind: ShaderKind,
        function: Function,
        interface: Vec<Rc<Op>>,
    ) -> Result<()> {
        let interface_words = {
            let mut out = Vec::new();

            for i in &interface {
                out.push(i.register_op(self)?.op_id(self)?.ok_or(ErrorKind::NoOp)?);
            }

            out
        };

        let name = function.name.clone();
        let id = function.register_function(self)?;

        self.builder.entry_point(
            kind.as_execution_model(),
            id,
            name.clone(),
            &interface_words,
        );

        #[cfg(feature = "vulkan")]
        {
            let interface = self::vulkan::interface_from_ops(name.clone(), kind, &interface)?;
            self.vulkan_shader_interfaces.push(interface);
        }

        Ok(())
    }

    pub fn module(self) -> rspirv::mr::Module {
        self.builder.module()
    }
}

#[cfg(feature = "vulkan")]
mod vulkan {
    use super::{Op, Rc};
    use errors::*;
    use interface::Interface;
    use rspirv::binary::Assemble;
    use std::borrow::Cow;
    use std::cmp;
    use std::collections::HashMap;
    use std::ffi::{CStr, CString};
    use std::slice;
    use std::sync::Arc;
    use vulkano::descriptor::descriptor::{DescriptorDesc, ShaderStages};
    use vulkano::descriptor::pipeline_layout::{PipelineLayoutDesc, PipelineLayoutDescPcRange};
    use vulkano::pipeline::shader::{GraphicsEntryPoint, GraphicsShaderType, ShaderInterfaceDef,
                                    ShaderInterfaceDefEntry};

    // patch implementation of ShaderKind.
    impl super::ShaderKind {
        pub fn to_shader_stages(self) -> ShaderStages {
            use super::ShaderKind::*;

            match self {
                Vertex => ShaderStages {
                    vertex: true,
                    ..ShaderStages::none()
                },
                Fragment => ShaderStages {
                    fragment: true,
                    ..ShaderStages::none()
                },
            }
        }

        pub fn to_shader_type(self) -> GraphicsShaderType {
            use super::ShaderKind::*;

            match self {
                Vertex => GraphicsShaderType::Vertex,
                Fragment => GraphicsShaderType::Fragment,
            }
        }
    }

    // patch implementation of Shader.
    impl super::Shader {
        pub fn vulkan_shader_module(
            self,
            device: ::std::sync::Arc<::vulkano::device::Device>,
        ) -> Result<VulkanShader> {
            use vulkano::pipeline::shader::ShaderModule;

            let mut entry_points = HashMap::new();

            for interface in self.vulkan_shader_interfaces {
                entry_points.insert(interface.name.clone(), interface);
            }

            let module = self.builder.module();
            let code = module.assemble();

            let module = unsafe {
                let code = slice::from_raw_parts(code.as_ptr() as *const u8, code.len() * 4);
                ShaderModule::new(device, &code)?
            };

            Ok(VulkanShader {
                module: module,
                entry_points: entry_points,
            })
        }
    }

    #[derive(Debug)]
    pub struct ShaderInterface {
        name: String,
        name_cstring: CString,
        kind: super::ShaderKind,
        input: ShaderInput,
        output: ShaderOutput,
        layout: ShaderLayout,
    }

    #[derive(Debug)]
    pub struct VulkanShader {
        module: Arc<::vulkano::pipeline::shader::ShaderModule>,
        entry_points: HashMap<String, ShaderInterface>,
    }

    impl VulkanShader {
        pub fn graphics_entry_point(
            &self,
            name: &str,
        ) -> Option<GraphicsEntryPoint<(), ShaderInput, ShaderOutput, ShaderLayout>> {
            if let Some(interface) = self.entry_points.get(name) {
                let entry_point = unsafe {
                    let name = CStr::from_ptr(interface.name_cstring.as_ptr());

                    self.module.graphics_entry_point(
                        name,
                        interface.input.clone(),
                        interface.output.clone(),
                        interface.layout.clone(),
                        interface.kind.to_shader_type(),
                    )
                };

                return Some(entry_point);
            }

            None
        }
    }

    #[derive(Debug, Clone)]
    pub struct ShaderInput {
        input: Vec<ShaderInterfaceDefEntry>,
    }

    unsafe impl ShaderInterfaceDef for ShaderInput {
        type Iter = ::std::vec::IntoIter<ShaderInterfaceDefEntry>;

        fn elements(&self) -> Self::Iter {
            self.input.clone().into_iter()
        }
    }

    #[derive(Debug, Clone)]
    pub struct ShaderOutput {
        output: Vec<ShaderInterfaceDefEntry>,
    }

    unsafe impl ShaderInterfaceDef for ShaderOutput {
        type Iter = ::std::vec::IntoIter<ShaderInterfaceDefEntry>;

        fn elements(&self) -> Self::Iter {
            self.output.clone().into_iter()
        }
    }

    #[derive(Debug, Clone)]
    pub struct ShaderLayout {
        /// Number of sets.
        num_sets: usize,
        /// Bindings in sets.
        bindings: HashMap<usize, usize>,
        /// Descriptors
        descriptors: HashMap<(usize, usize), DescriptorDesc>,
        /// Number of push constants.
        num_push_constants_ranges: usize,
        /// Push constant ranges.
        push_constants_range: HashMap<usize, PipelineLayoutDescPcRange>,
    }

    unsafe impl PipelineLayoutDesc for ShaderLayout {
        fn num_sets(&self) -> usize {
            self.num_sets
        }

        fn num_bindings_in_set(&self, set: usize) -> Option<usize> {
            self.bindings.get(&set).cloned()
        }

        fn descriptor(&self, set: usize, binding: usize) -> Option<DescriptorDesc> {
            self.descriptors.get(&(set, binding)).cloned()
        }

        fn num_push_constants_ranges(&self) -> usize {
            self.num_push_constants_ranges
        }

        fn push_constants_range(&self, num: usize) -> Option<PipelineLayoutDescPcRange> {
            self.push_constants_range.get(&num).cloned()
        }
    }

    pub fn interface_from_ops(
        name: String,
        kind: super::ShaderKind,
        ops: &Vec<Rc<Op>>,
    ) -> Result<ShaderInterface> {
        use self::Interface::*;

        let mut num_sets = 0usize;
        let mut bindings = HashMap::new();
        let mut descriptors = HashMap::new();
        let mut input = Vec::new();
        let mut output = Vec::new();

        let stages = kind.to_shader_stages();

        for op in ops {
            let interface = op.as_interface().ok_or(ErrorKind::NotInterface)?;

            let (dest, location, name, dest_type) = match interface {
                Input { var, location } => (
                    &mut input,
                    location,
                    var.name.to_owned(),
                    &var.ty.pointee_type,
                ),
                Output { var, location } => (
                    &mut output,
                    location,
                    var.name.to_owned(),
                    &var.ty.pointee_type,
                ),
                Uniform { var, binding, set } => {
                    let descriptor = var.as_vulkan_descriptor(&stages).ok_or(
                        ErrorKind::IllegalInterfaceType,
                    )?;

                    let set = set as usize;
                    let binding = binding as usize;

                    num_sets = cmp::max(set, num_sets);
                    bindings.insert(set, binding);
                    descriptors.insert((set, binding), descriptor);
                    continue;
                }
                BuiltIn => continue,
            };

            let format = dest_type.as_vulkano_format().ok_or(
                ErrorKind::IllegalInterfaceType,
            )?;

            dest.push(ShaderInterfaceDefEntry {
                location: location..location + 1,
                format: format,
                name: Some(Cow::Owned(name)),
            });
        }

        let num_push_constants_ranges = 0;
        let push_constants_range = HashMap::new();

        Ok(ShaderInterface {
            name: name.clone(),
            name_cstring: CString::new(name.clone())?,
            kind: kind,
            input: ShaderInput { input: input },
            output: ShaderOutput { output: output },
            layout: ShaderLayout {
                num_sets: num_sets,
                bindings: bindings,
                descriptors: descriptors,
                num_push_constants_ranges: num_push_constants_ranges,
                push_constants_range: push_constants_range,
            },
        })
    }
}
