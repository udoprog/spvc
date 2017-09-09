use super::errors::*;
use super::reg_var::RegVar;
use super::shader::Shader;
use super::spirv_type::SpirvType;
use super::storage_class::StorageClass;
use std::fmt;

pub trait Var: fmt::Debug {
    fn storage_class(&self) -> Option<StorageClass> {
        None
    }

    fn var_type(&self) -> &SpirvType;

    fn register_var(&self, shader: &mut Shader) -> Result<Box<RegVar>>;
}
