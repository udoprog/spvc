//! Utilities for programmatically build shaders for SPIR-V
#![warn(missing_docs)]
#![recursion_limit="128"]

#[cfg(feature = "vulkan")]
extern crate vulkano;
extern crate rspirv;
extern crate spirv_headers as spirv;
#[macro_use]
extern crate error_chain;

mod access;
mod function;
mod uniform_var;
mod input_var;
mod output_var;
mod built_in_var;
mod ops;
mod pointer;
mod types;
mod reg_op;
mod shader;
mod spirv_type;
mod op;
mod matrix_dims;
mod vector_dims;
mod interface;
mod op_key;
pub mod errors;
pub mod struct_member;

pub use self::access::AccessTrait;
pub use self::built_in_var::BuiltInVar;
pub use self::function::FunctionBuilder;
pub use self::input_var::InputVar;
// FIXME: Too many to list explicitly.
pub use self::ops::*;
pub use self::output_var::OutputVar;
pub use self::shader::{Shader, ShaderKind};
pub use self::spirv::BuiltIn;
pub use self::spirv::StorageClass;
pub use self::struct_member::StructMember;
pub use self::types::{Bool, Float, Matrix, Struct, Vector, mat3, mat4, st, vec2, vec3, vec4};
pub use self::uniform_var::UniformVar;
