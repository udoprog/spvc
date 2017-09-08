use super::errors::*;
use super::shader::Shader;
use super::spirv::Word;
use super::spirv_type::SpirvType;
use super::storage_class::StorageClass;
use std::rc::Rc;

#[derive(Debug)]
pub struct Pointer(pub StorageClass, pub Rc<SpirvType>);

impl SpirvType for Pointer {
    fn register_type(&self, shader: &mut Shader) -> Result<Word> {
        let pointee_type = self.1.register_type(shader)?;
        let pointer_type = shader.register_pointer_type(self.0, pointee_type)?;
        Ok(pointer_type)
    }

    fn width(&self) -> u32 {
        4
    }
}
