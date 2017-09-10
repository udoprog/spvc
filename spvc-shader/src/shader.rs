use super::errors::*;
use super::function::Function;
use super::op::Op;
use super::rspirv;
use super::spirv::{ExecutionModel, Word};
use super::spirv_type::SpirvType;
use super::storage_class::StorageClass;
use super::type_key::TypeKey;
use super::types::{Float, UnsignedInteger};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub struct Shader {
    /// Internal builder
    pub(crate) builder: rspirv::mr::Builder,
    /// Cached types, to only initialize each type once.
    type_cache: HashMap<TypeKey, Word>,
    #[cfg(vulkan)]
    vulkano_entry_points: Vec<self::vulkano::VulkanoShader>,
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
            #[cfg(vulkan)]
            vulkano_entry_points: Vec::new(),
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

    pub fn vertex_entry_point(
        &mut self,
        function: Function,
        interface: Vec<Rc<Box<Op>>>,
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
            ExecutionModel::Vertex,
            id,
            name,
            interface_words,
        );

        #[cfg(vulkan)]
        {
            self.vulkano_entry_points.push(
                self::vulkano::VulkanoShader::from_ops(function.name.as_str(), &interface)?,
            );
        }

        Ok(())
    }

    pub fn module(self) -> rspirv::mr::Module {
        self.builder.module()
    }
}

#[cfg(feature = "vulkan")]
mod vulkano {
    use super::{Op, Rc};
    use errors::*;
    use rspirv::binary::Assemble;
    use std::borrow::Cow;
    use std::slice;
    use std::sync::Arc;
    use vulkano::pipeline::shader::{ShaderInterfaceDef, ShaderInterfaceDefEntry};

    impl super::Shader {
        pub fn vulkan_shader_module(
            self,
            device: ::std::sync::Arc<::vulkano::device::Device>,
        ) -> Result<Arc<::vulkano::pipeline::shader::ShaderModule>> {
            use vulkano::pipeline::shader::ShaderModule;

            let module = self.module();
            let code = module.assemble();

            let module = unsafe {
                let code = slice::from_raw_parts(code.as_ptr() as *const u8, code.len() * 4);
                ShaderModule::new(device, &code)?
            };

            Ok(module)
        }
    }

    #[derive(Debug)]
    pub struct VulkanoShader {
        entries: Vec<ShaderInterfaceDefEntry>,
    }

    impl VulkanoShader {
        pub fn from_ops(name: &str, ops: &Vec<Rc<Box<Op>>>) -> Result<VulkanoShader> {
            let mut entries = Vec::new();

            for op in ops {
                let interface = op.as_interface().ok_or(ErrorKind::NotInterface)?;

                let format = op.op_type().as_vulkano_format().ok_or(
                    ErrorKind::IllegalInterfaceType,
                )?;

                entries.push(ShaderInterfaceDefEntry {
                    location: interface.location..interface.location + 1,
                    format: format,
                    name: Some(Cow::Owned(String::from(name))),
                });
            }

            Ok(VulkanoShader { entries: entries })
        }
    }

    unsafe impl ShaderInterfaceDef for VulkanoShader {
        type Iter = ::std::vec::IntoIter<ShaderInterfaceDefEntry>;

        fn elements(&self) -> Self::Iter {
            self.entries.clone().into_iter()
        }
    }
}
