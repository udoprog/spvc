use super::errors::*;
use super::shader::Shader;
use super::spirv::Word;
use std::fmt;

/// Describes a type in SPIR-V.
pub trait SpirvType: fmt::Debug {
    fn register_type(&self, shader: &mut Shader) -> Result<Word>;

    /// Hook to register extra directives when this type is the member of a struct.
    fn register_struct_extra(&self, _id: Word, _index: u32, _shader: &mut Shader) -> Result<()> {
        Ok(())
    }

    fn width(&self) -> u32;
}
