use super::errors::*;
use super::registered_statement::RegisteredStatement;
use super::shader::Shader;
use std::fmt;

pub trait Statement: fmt::Debug {
    fn register_statement(&self, shader: &mut Shader) -> Result<Box<RegisteredStatement>>;
}
