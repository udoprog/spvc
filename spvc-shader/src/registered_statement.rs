use super::errors::*;
use super::shader::Shader;
use super::spirv::Word;
use std::fmt;

/// Reflects a registered single statement that results in an object.
pub trait RegisteredStatement: fmt::Debug {
    fn statement_id(&self, shader: &mut Shader) -> Result<Word>;
}
