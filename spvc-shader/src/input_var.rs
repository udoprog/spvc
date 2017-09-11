use super::errors::*;
use super::interface::Interface;
use super::op::Op;
use super::pointer::Pointer;
use super::reg_op::RegOp;
use super::rspirv::mr::Operand;
use super::shader::Shader;
use super::spirv::{Decoration, StorageClass};
use super::spirv_type::{SpirvType, WrapperType};
use super::type_key::TypeKey;
use std::rc::Rc;

#[derive(Debug)]
pub struct InputVar {
    pub name: String,
    pub ty: Pointer,
    pub location: u32,
}

impl Op for InputVar {
    fn as_interface(&self) -> Option<Interface> {
        Some(Interface::Input {
            var: self,
            location: self.location,
        })
    }

    fn storage_class(&self) -> Option<StorageClass> {
        Some(StorageClass::Input)
    }

    fn op_type(&self) -> &SpirvType {
        &self.ty
    }

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
        let variable_type = self.ty.register_type(shader)?;

        let id = shader.cached_type(
            TypeKey::InputVar {
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
    pub fn new<T: 'static + SpirvType>(name: &str, ty: T, location: u32) -> Rc<InputVar> {
        Rc::new(InputVar {
            name: String::from(name),
            ty: Pointer::new(StorageClass::Input, Rc::new(ty)),
            location: location,
        })
    }

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

impl WrapperType for InputVar {
    fn wrapped_type(&self) -> &SpirvType {
        &self.ty
    }
}
