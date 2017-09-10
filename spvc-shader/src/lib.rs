#![recursion_limit="128"]

#[cfg(feature = "vulkan")]
extern crate vulkano;
extern crate rspirv;
extern crate spirv_headers as spirv;
#[macro_use]
extern crate error_chain;

mod access;
mod function;
mod global_var;
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
mod storage_class;
mod type_key;
pub mod errors;
pub mod struct_member;

pub use self::access::AccessTrait;
pub use self::function::FunctionBuilder;
pub use self::global_var::GlobalVar;
pub use self::ops::*;
pub use self::shader::Shader;
pub use self::storage_class::StorageClass;
pub use self::struct_member::StructMember;
pub use self::types::{Bool, Float, Matrix, Struct, Vector, mat3, mat4, st, vec2, vec3, vec4};
