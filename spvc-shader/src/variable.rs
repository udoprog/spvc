use super::errors::*;
use super::registered_variable::RegisteredVariable;
use super::shader::Shader;
use super::spirv_type::SpirvType;
use super::storage_class::StorageClass;
use std::fmt;

pub trait Variable: fmt::Debug {
    fn storage_class(&self) -> Option<StorageClass> {
        None
    }

    fn variable_type(&self) -> &SpirvType;

    fn register_variable(&self, shader: &mut Shader) -> Result<Box<RegisteredVariable>>;
}
