
use super::errors::*;
use super::registered_load::RegisteredLoad;
use super::shader::Shader;
use std::fmt;

/// Reflects a type that might need loading (through OpLoad) before it can be used.
pub trait Load: fmt::Debug {
    type LoadedType;

    fn loaded_type(&self) -> &Self::LoadedType;

    fn register_load(&self, shader: &mut Shader) -> Result<Box<RegisteredLoad>>;
}
