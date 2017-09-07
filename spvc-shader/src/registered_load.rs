use super::errors::*;
use super::shader::Shader;
use super::spirv::Word;
use std::fmt;

/// Reflects an object that might need to be loaded through OpLoad before it can be used.
pub trait RegisteredLoad: fmt::Debug {
    fn load(&self, shader: &mut Shader) -> Result<Word>;
}
