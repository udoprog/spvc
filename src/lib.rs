#[macro_use]
extern crate error_chain;
extern crate rspirv;
extern crate spirv_headers;
extern crate glsl_struct;

pub mod spirv_shader_base;
pub mod errors;

pub use self::errors::Error;
