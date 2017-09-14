use super::errors::*;
use super::interface::Interface;
use super::op::Op;
use super::op_key::OpKey;
use super::pointer::Pointer;
use super::reg_op::RegOp;
use super::rspirv::mr::Operand;
use super::shader::Shader;
use super::spirv::{Decoration, StorageClass};
use super::spirv_type::{SpirvType, WrapperType};
use std::rc::Rc;

/// Reflection of a uniform global variable.
#[derive(Debug)]
pub struct UniformVar {
    /// Name of the variable.
    pub name: String,
    /// Type of the variable, packed behind a pointer.
    pub pointer: Pointer,
    /// The set of the uniform variable.
    pub set: u32,
    /// The binding of the uniform variable.
    pub binding: u32,
}

impl WrapperType for UniformVar {
    fn wrapped_type(&self) -> &SpirvType {
        &self.pointer
    }
}

impl Op for UniformVar {
    fn as_interface(&self) -> Option<Interface> {
        Some(Interface::Uniform(self))
    }

    fn storage_class(&self) -> Option<StorageClass> {
        Some(StorageClass::Uniform)
    }

    fn op_type(&self) -> &SpirvType {
        &self.pointer
    }

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
        let variable_type = self.pointer.register_type(shader)?;

        let id = shader.cache_op(
            OpKey::UniformVar {
                variable_type: variable_type,
                set: self.set,
                binding: self.binding,
            },
            |s| {
                let variable_id = s.builder.variable(
                    variable_type,
                    None,
                    StorageClass::Uniform,
                    None,
                );

                s.name(variable_id, self.name.as_str());

                s.builder.decorate(
                    variable_id,
                    Decoration::DescriptorSet,
                    &[Operand::LiteralInt32(self.set)],
                );

                s.builder.decorate(
                    variable_id,
                    Decoration::Binding,
                    &[Operand::LiteralInt32(self.binding)],
                );

                Ok(variable_id)
            },
        )?;

        Ok(Box::new(id))
    }
}

impl UniformVar {
    /// Construct a new uniform variable.
    pub fn new<T: 'static + SpirvType>(
        name: &str,
        ty: T,
        set: u32,
        binding: u32,
    ) -> Rc<UniformVar> {
        Rc::new(UniformVar {
            name: String::from(name),
            pointer: Pointer::new(StorageClass::Uniform, Rc::new(ty)),
            set: set,
            binding: binding,
        })
    }

    /// Setup a vulkan descriptor for this uniform variable.
    #[cfg(feature = "vulkan")]
    pub fn as_vulkan_descriptor(
        &self,
        stages: &::vulkano::descriptor::descriptor::ShaderStages,
    ) -> Option<::vulkano::descriptor::descriptor::DescriptorDesc> {
        use vulkano::descriptor::descriptor::{DescriptorBufferDesc, DescriptorDesc,
                                              DescriptorDescTy};

        Some(DescriptorDesc {
            ty: DescriptorDescTy::Buffer(DescriptorBufferDesc {
                dynamic: Some(false),
                storage: false,
            }),
            array_count: 1,
            stages: stages.clone(),
            readonly: true,
        })
    }
}
