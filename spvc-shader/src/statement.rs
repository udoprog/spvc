use super::errors::*;
use super::registered_statement::RegisteredStatement;
use super::shader::Shader;
use super::spirv_type::SpirvType;
use std::fmt;

pub trait Statement: fmt::Debug {
    fn statement_type(&self) -> &SpirvType;

    fn register_statement(&self, shader: &mut Shader) -> Result<Box<RegisteredStatement>>;
}
