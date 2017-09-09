use super::errors::*;
use super::reg_op::RegOp;
use super::shader::Shader;
use super::spirv_type::SpirvType;
use std::fmt;

pub trait Op: fmt::Debug {
    fn op_type(&self) -> &SpirvType;

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>>;
}
