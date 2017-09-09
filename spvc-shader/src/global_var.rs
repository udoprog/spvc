use super::errors::*;
use super::op::Op;
use super::reg_op::RegOp;
use super::rspirv::mr::Operand;
use super::shader::Shader;
use super::spirv::{BuiltIn, Decoration, Word};
use super::spirv_type::SpirvType;
use super::storage_class::StorageClass;
use super::type_key::TypeKey;
use std::rc::Rc;

#[derive(Debug)]
pub struct GlobalVar {
    name: String,
    storage_class: StorageClass,
    ty: Rc<SpirvType>,
    set: Option<u32>,
    binding: Option<u32>,
    location: Option<u32>,
    built_in: Option<BuiltIn>,
}

impl Op for GlobalVar {
    fn storage_class(&self) -> Option<StorageClass> {
        Some(self.storage_class)
    }

    fn op_type(&self) -> &SpirvType {
        self.ty.as_ref()
    }

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
        let pointee_type = self.ty.register_type(shader)?;

        let variable_type = shader.register_pointer_type(
            self.storage_class,
            pointee_type,
        )?;

        let id = shader.cached_type(
            TypeKey::GlobalVar {
                storage_class: self.storage_class,
                variable_type: variable_type,
                set: self.set.clone(),
                binding: self.binding.clone(),
                location: self.location.clone(),
            },
            |s| {
                let variable_id = s.builder.variable(
                    variable_type,
                    None,
                    self.storage_class.into(),
                    None,
                );

                s.name(variable_id, self.name.as_str());

                if let Some(set) = self.set {
                    s.builder.decorate(
                        variable_id,
                        Decoration::DescriptorSet,
                        vec![Operand::LiteralInt32(set)],
                    );
                }

                if let Some(binding) = self.binding {
                    s.builder.decorate(
                        variable_id,
                        Decoration::Binding,
                        vec![Operand::LiteralInt32(binding)],
                    );
                }

                if let Some(location) = self.location {
                    s.builder.decorate(
                        variable_id,
                        Decoration::Location,
                        vec![Operand::LiteralInt32(location)],
                    );
                }

                if let Some(built_in) = self.built_in {
                    s.builder.decorate(
                        variable_id,
                        Decoration::BuiltIn,
                        vec![Operand::BuiltIn(built_in)],
                    );
                }

                Ok(variable_id)
            },
        )?;

        Ok(Box::new(id))
    }
}

impl GlobalVar {
    pub fn new<T: 'static + SpirvType>(
        name: &str,
        ty: T,
        storage_class: StorageClass,
    ) -> GlobalVar {
        GlobalVar {
            name: String::from(name),
            storage_class: storage_class,
            ty: Rc::new(ty),
            set: None,
            binding: None,
            location: None,
            built_in: None,
        }
    }

    pub fn with_set(self, set: u32) -> GlobalVar {
        GlobalVar {
            name: self.name,
            storage_class: self.storage_class,
            ty: self.ty,
            set: Some(set),
            binding: self.binding,
            location: self.location,
            built_in: self.built_in,
        }
    }

    pub fn with_binding(self, binding: u32) -> GlobalVar {
        GlobalVar {
            name: self.name,
            storage_class: self.storage_class,
            ty: self.ty,
            set: self.set,
            binding: Some(binding),
            location: self.location,
            built_in: self.built_in,
        }
    }

    pub fn with_location(self, location: u32) -> GlobalVar {
        GlobalVar {
            name: self.name,
            storage_class: self.storage_class,
            ty: self.ty,
            set: self.set,
            binding: self.binding,
            location: Some(location),
            built_in: self.built_in,
        }
    }

    pub fn with_built_in(self, built_in: BuiltIn) -> GlobalVar {
        GlobalVar {
            name: self.name,
            storage_class: self.storage_class,
            ty: self.ty,
            set: self.set,
            binding: self.binding,
            location: self.location,
            built_in: Some(built_in),
        }
    }

    pub fn build(self) -> Rc<Box<Op>> {
        Rc::new(Box::new(self))
    }
}

impl SpirvType for GlobalVar {
    fn register_type(&self, shader: &mut Shader) -> Result<Word> {
        let pointee_type = self.ty.register_type(shader)?;
        shader.register_pointer_type(self.storage_class, pointee_type)
    }

    fn width(&self) -> u32 {
        4
    }
}
