use super::errors::*;
use super::shader::Shader;
use super::spirv::Word;
use std::fmt;

pub trait RegVar: fmt::Debug {
    fn var_id(&self, shader: &mut Shader) -> Result<Word>;
}

impl RegVar for Word {
    fn var_id(&self, _: &mut Shader) -> Result<Word> {
        Ok(*self)
    }
}
