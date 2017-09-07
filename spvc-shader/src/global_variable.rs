use super::access::Access;
use super::errors::*;
use super::glsl_struct::GlslStruct;
use super::glsl_struct_member::GlslStructMember;
use super::pointer::Pointer;
use super::registered_variable::RegisteredVariable;
use super::shader::Shader;
use super::spirv::Word;
use super::spirv_type::SpirvType;
use super::storage_class::StorageClass;
use super::variable::Variable;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct GlobalVariable<T> {
    name: String,
    storage_class: StorageClass,
    ty: Rc<T>,
    set: Option<u32>,
    binding: Option<u32>,
    location: Option<u32>,
}

impl<T: SpirvType> Variable for GlobalVariable<T> {
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

impl<T> GlobalVariable<T>
where
    T: SpirvType,
{
    pub fn new(name: &str, ty: T, storage_class: StorageClass) -> GlobalVariable<T> {
        GlobalVariable {
            name: String::from(name),
            storage_class: storage_class,
            ty: Rc::new(ty),
            set: None,
            binding: None,
            location: None,
        }
    }

    pub fn with_set(self, set: u32) -> GlobalVariable<T> {
        GlobalVariable {
            name: self.name,
            storage_class: self.storage_class,
            ty: self.ty,
            set: Some(set),
            binding: self.binding,
            location: self.location,
        }
    }

    pub fn with_binding(self, binding: u32) -> GlobalVariable<T> {
        GlobalVariable {
            name: self.name,
            storage_class: self.storage_class,
            ty: self.ty,
            set: self.set,
            binding: Some(binding),
            location: self.location,
        }
    }

    pub fn with_location(self, location: u32) -> GlobalVariable<T> {
        GlobalVariable {
            name: self.name,
            storage_class: self.storage_class,
            ty: self.ty,
            set: self.set,
            binding: self.binding,
            location: Some(location),
        }
    }
}

impl<T: fmt::Debug> GlobalVariable<GlslStruct<T>> {
    pub fn access<M: SpirvType>(
        &self,
        member: GlslStructMember<T, M>,
    ) -> Access<GlobalVariable<GlslStruct<T>>, M> {
        let base = GlobalVariable {
            name: self.name.clone(),
            storage_class: self.storage_class,
            ty: self.ty.clone(),
            set: self.set,
            binding: self.binding,
            location: self.location,
        };

        Access {
            base: base,
            pointer_type: Pointer(self.storage_class, member.ty.clone()),
            loaded_type: member.ty.clone(),
            index: member.index,
        }
    }
}

impl<T: SpirvType> SpirvType for GlobalVariable<T> {
    fn register_type(&self, shader: &mut Shader) -> Result<Word> {
        let pointee_type = self.ty.register_type(shader)?;
        shader.register_pointer_type(self.storage_class, pointee_type)
    }

    fn width(&self) -> u32 {
        4
    }
}
