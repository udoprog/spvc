extern crate rspirv;
extern crate spirv_headers as spirv;
#[macro_use]
extern crate error_chain;

mod access;
mod function;
mod global_variable;
mod ops;
mod pointer;
mod primitives;
mod registered_statement;
mod registered_variable;
mod shader;
mod spirv_type;
mod statement;
mod storage_class;
mod type_key;
mod variable;
pub mod errors;
pub mod glsl_struct;
pub mod glsl_struct_member;

pub type GlslVec4 = [f32; 4];
pub type GlslMat4 = [GlslVec4; 4];
pub type GlslBool = u32;

pub use self::access::AccessTrait;
pub use self::function::FunctionBuilder;
pub use self::global_variable::GlobalVariable;
pub use self::ops::{Load, Mul, Store};
pub use self::primitives::Bool;
pub use self::primitives::Float;
pub use self::primitives::Matrix;
pub use self::primitives::Vector;
pub use self::shader::Shader;
pub use self::storage_class::StorageClass;

/// Corresponds to the GLSL type vec2.
pub fn vec2() -> Vector {
    Vector::new(Float, 3)
}

/// Corresponds to the GLSL type vec3.
pub fn vec3() -> Vector {
    Vector::new(Float, 3)
}

/// Corresponds to the GLSL type vec4.
pub fn vec4() -> Vector {
    Vector::new(Float, 4)
}

/// Corresponds to the GLSL type mat4.
pub fn mat4() -> Matrix {
    Matrix::new(Vector::new(Float, 4), 4)
}
