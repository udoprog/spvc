use super::errors::*;
use super::interface::Interface;
use super::op::Op;
use super::pointer::Pointer;
use super::reg_op::RegOp;
use super::rspirv::mr::Operand;
use super::shader::Shader;
use super::spirv::{BuiltIn, Decoration, StorageClass};
use super::spirv_type::{SpirvType, WrapperType};
use super::type_key::TypeKey;
use std::rc::Rc;

/// Represents built-in variables.
#[derive(Debug)]
pub struct BuiltInVar {
    pub name: String,
    storage_class: StorageClass,
    pub ty: Pointer,
    built_in: BuiltIn,
}

impl WrapperType for BuiltInVar {
    fn wrapped_type(&self) -> &SpirvType {
        &self.ty
    }
}

impl Op for BuiltInVar {
    fn as_interface(&self) -> Option<Interface> {
        return Some(Interface::BuiltIn);
    }

    fn storage_class(&self) -> Option<StorageClass> {
        Some(self.storage_class)
    }

    fn op_type(&self) -> &SpirvType {
        &self.ty
    }

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
        let variable_type = self.ty.register_type(shader)?;

        let id = shader.cached_type(
            TypeKey::BuiltInVar {
                storage_class: self.storage_class,
                variable_type: variable_type,
                built_in: self.built_in.clone(),
            },
            |s| {
                let variable_id = s.builder.variable(
                    variable_type,
                    None,
                    self.storage_class.into(),
                    None,
                );

                s.name(variable_id, self.name.as_str());

                s.builder.decorate(
                    variable_id,
                    Decoration::BuiltIn,
                    &[Operand::BuiltIn(self.built_in)],
                );

                Ok(variable_id)
            },
        )?;

        Ok(Box::new(id))
    }
}

impl BuiltInVar {
    pub fn new<T: 'static + SpirvType>(
        name: &str,
        ty: T,
        storage_class: StorageClass,
        built_in: BuiltIn,
    ) -> Rc<BuiltInVar> {
        Rc::new(BuiltInVar {
            name: String::from(name),
            storage_class: storage_class,
            ty: Pointer::new(storage_class, Rc::new(ty)),
            built_in: built_in,
        })
    }
}
