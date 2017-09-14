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

/// Reflection of an output variable.
#[derive(Debug)]
pub struct OutputVar {
    /// Name of the output variable.
    pub name: String,
    /// Type of the output variable, packed into a pointer.
    pub pointer: Pointer,
    /// Location of the output variable.
    pub location: u32,
}

impl WrapperType for OutputVar {
    fn wrapped_type(&self) -> &SpirvType {
        &self.pointer
    }
}

impl Op for OutputVar {
    fn as_interface(&self) -> Option<Interface> {
        Some(Interface::Output(self))
    }

    fn storage_class(&self) -> Option<StorageClass> {
        Some(StorageClass::Output)
    }

    fn op_type(&self) -> &SpirvType {
        &self.pointer
    }

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
        let variable_type = self.pointer.register_type(shader)?;

        let id = shader.cache_op(
            OpKey::OutputVar {
                variable_type: variable_type,
                location: self.location,
            },
            |s| {
                let variable_id = s.builder.variable(
                    variable_type,
                    None,
                    StorageClass::Output,
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

impl OutputVar {
    /// Create a new output variable.
    pub fn new<T: 'static + SpirvType>(name: &str, ty: T, location: u32) -> Rc<OutputVar> {
        Rc::new(OutputVar {
            name: String::from(name),
            pointer: Pointer::new(StorageClass::Output, Rc::new(ty)),
            location: location,
        })
    }

    /// Create a vulkan descriptor for this output variable.
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
            readonly: false,
        })
    }
}
