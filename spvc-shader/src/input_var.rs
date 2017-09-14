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

/// Reflection of an input variable.
#[derive(Debug)]
pub struct InputVar {
    /// Name of the input variable.
    pub name: String,
    /// Type of the input variable, packed into a pointer.
    pub pointer: Pointer,
    /// Location of the input variable.
    pub location: u32,
}

impl WrapperType for InputVar {
    fn wrapped_type(&self) -> &SpirvType {
        &self.pointer
    }
}

impl Op for InputVar {
    fn as_interface(&self) -> Option<Interface> {
        Some(Interface::Input(self))
    }

    fn storage_class(&self) -> Option<StorageClass> {
        Some(StorageClass::Input)
    }

    fn op_type(&self) -> &SpirvType {
        &self.pointer
    }

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
        let variable_type = self.pointer.register_type(shader)?;

        let id = shader.cache_op(
            OpKey::InputVar {
                variable_type: variable_type,
                location: self.location,
            },
            |s| {
                let variable_id = s.builder.variable(
                    variable_type,
                    None,
                    StorageClass::Input,
                    None,
                );

                s.name(variable_id, self.name.as_str());

                s.builder.decorate(
                    variable_id,
                    Decoration::Location,
                    &[Operand::LiteralInt32(self.location)],
                );

                Ok(variable_id)
            },
        )?;

        Ok(Box::new(id))
    }
}

impl InputVar {
    /// Create a new input variable.
    pub fn new<T: 'static + SpirvType>(name: &str, ty: T, location: u32) -> Rc<InputVar> {
        Rc::new(InputVar {
            name: String::from(name),
            pointer: Pointer::new(StorageClass::Input, Rc::new(ty)),
            location: location,
        })
    }

    /// Create a descriptor for the input variable.
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
