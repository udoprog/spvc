use super::errors::*;
use super::shader::Shader;
use super::spirv::Word;
use std::fmt;

pub trait RegisteredVariable: fmt::Debug {
    fn variable_id(&self, shader: &mut Shader) -> Result<Word>;
}

impl RegisteredVariable for Word {
    fn variable_id(&self, _: &mut Shader) -> Result<Word> {
        Ok(*self)
    }
}
