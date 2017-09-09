use super::errors::*;
use super::shader::Shader;
use super::spirv::Word;
use super::spirv_type::SpirvType;
use super::storage_class::StorageClass;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Pointer {
    storage_class: StorageClass,
    pub pointee_type: Rc<Box<SpirvType>>,
}

impl Pointer {
    pub fn new(storage_class: StorageClass, pointee_type: Rc<Box<SpirvType>>) -> Pointer {
        Pointer {
            storage_class: storage_class,
            pointee_type: pointee_type,
        }
    }
}

impl SpirvType for Pointer {
    fn register_type(&self, shader: &mut Shader) -> Result<Word> {
        let pointee_type = self.pointee_type.register_type(shader)?;

        let pointer_type = shader.register_pointer_type(
            self.storage_class,
            pointee_type,
        )?;

        Ok(pointer_type)
    }

    fn as_pointer(&self) -> Option<Pointer> {
        Some(self.clone())
    }

    fn width(&self) -> u32 {
        4
    }

    fn matches(&self, other: &SpirvType) -> bool {
        if let Some(pointer) = other.as_pointer() {
            return self.pointee_type.matches(
                pointer.pointee_type.as_ref().as_ref(),
            );
        }

        false
    }

    fn display(&self) -> String {
        format!("*{}", self.pointee_type.display())
    }
}
