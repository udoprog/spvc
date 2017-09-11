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
pub struct UniformVar {
    pub name: String,
    pub ty: Pointer,
    pub set: u32,
    pub binding: u32,
}

impl WrapperType for UniformVar {
    fn wrapped_type(&self) -> &SpirvType {
        &self.ty
    }
}

impl Op for UniformVar {
    fn as_interface(&self) -> Option<Interface> {
        Some(Interface::Uniform {
            var: self,
            set: self.set,
            binding: self.binding,
        })
    }

    fn storage_class(&self) -> Option<StorageClass> {
        Some(StorageClass::Uniform)
    }

    fn op_type(&self) -> &SpirvType {
        &self.ty
    }

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
        let variable_type = self.ty.register_type(shader)?;

        let id = shader.cached_type(
            TypeKey::UniformVar {
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
    pub fn new<T: 'static + SpirvType>(
        name: &str,
        ty: T,
        set: u32,
        binding: u32,
    ) -> Rc<UniformVar> {
        Rc::new(UniformVar {
            name: String::from(name),
            ty: Pointer::new(StorageClass::Uniform, Rc::new(ty)),
            set: set,
            binding: binding,
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
