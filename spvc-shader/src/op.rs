use super::errors::*;
use super::interface::Interface;
use super::ops::BadOp;
use super::reg_op::RegOp;
use super::shader::Shader;
use super::spirv::StorageClass;
use super::spirv_type::SpirvType;
use std::fmt;
use std::rc::Rc;

pub trait Op: fmt::Debug {
    /// If this is an access operation, returns the base being accessed.
    fn base(&self) -> Option<&Rc<Op>> {
        None
    }

    fn access_chain(&self) -> Option<&[u32]> {
        None
    }

    fn storage_class(&self) -> Option<StorageClass> {
        None
    }

    fn op_type(&self) -> &SpirvType;

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>>;

    /// Convert this op to a bad op, if it is one.
    fn as_bad_op(&self) -> Option<&BadOp> {
        None
    }

    fn as_interface(&self) -> Option<Interface> {
        None
    }
}
