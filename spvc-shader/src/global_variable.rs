use super::access::Access;
use super::errors::*;
use super::glsl_struct_member::GlslStructMember;
use super::pointer::Pointer;
use super::registered_variable::RegisteredVariable;
use super::shader::Shader;
use super::spirv::Word;
use super::spirv_type::SpirvType;
use super::statement::Statement;
use super::storage_class::StorageClass;
use super::variable::Variable;
use std::rc::Rc;

#[derive(Debug)]
pub struct GlobalVariable {
    name: String,
    storage_class: StorageClass,
    ty: Rc<SpirvType>,
    set: Option<u32>,
    binding: Option<u32>,
    location: Option<u32>,
}

impl Variable for GlobalVariable {
    fn register_variable(&self, shader: &mut Shader) -> Result<Box<RegisteredVariable>> {
        let id = shader.global_variable(
            self.storage_class,
            self.ty.as_ref(),
            self.set,
            self.binding,
            self.location,
        )?;

        Ok(Box::new(id))
    }
}

impl GlobalVariable {
    pub fn new<T: 'static + SpirvType>(
        name: &str,
        ty: T,
        storage_class: StorageClass,
    ) -> GlobalVariable {
        GlobalVariable {
            name: String::from(name),
            storage_class: storage_class,
            ty: Rc::new(ty),
            set: None,
            binding: None,
            location: None,
        }
    }

    pub fn with_set(self, set: u32) -> GlobalVariable {
        GlobalVariable {
            name: self.name,
            storage_class: self.storage_class,
            ty: self.ty,
            set: Some(set),
            binding: self.binding,
            location: self.location,
        }
    }

    pub fn with_binding(self, binding: u32) -> GlobalVariable {
        GlobalVariable {
            name: self.name,
            storage_class: self.storage_class,
            ty: self.ty,
            set: self.set,
            binding: Some(binding),
            location: self.location,
        }
    }

    pub fn with_location(self, location: u32) -> GlobalVariable {
        GlobalVariable {
            name: self.name,
            storage_class: self.storage_class,
            ty: self.ty,
            set: self.set,
            binding: self.binding,
            location: Some(location),
        }
    }

    pub fn access(&self, member: GlslStructMember) -> Rc<Box<Statement>> {
        let base = GlobalVariable {
            name: self.name.clone(),
            storage_class: self.storage_class,
            ty: self.ty.clone(),
            set: self.set,
            binding: self.binding,
            location: self.location,
        };

        Rc::new(Box::new(Access {
            base: base,
            pointer_type: Pointer(self.storage_class, member.ty.clone()),
            accessed_type: member.ty.clone(),
            index: member.index,
        }))
    }
}

impl SpirvType for GlobalVariable {
    fn register_type(&self, shader: &mut Shader) -> Result<Word> {
        let pointee_type = self.ty.register_type(shader)?;
        shader.register_pointer_type(self.storage_class, pointee_type)
    }

    fn width(&self) -> u32 {
        4
    }
}
