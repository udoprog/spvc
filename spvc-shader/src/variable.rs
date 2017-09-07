use super::errors::*;
use super::registered_variable::RegisteredVariable;
use super::shader::Shader;
use std::fmt;

pub trait Variable: fmt::Debug {
    fn register_variable(&self, shader: &mut Shader) -> Result<Box<RegisteredVariable>>;
}
