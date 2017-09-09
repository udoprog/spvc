use super::errors::*;
use super::rspirv;
use super::shader::Shader;
use super::spirv::{Decoration, Word};
use super::spirv_type::SpirvType;
use super::struct_member::StructMember;
use super::type_key::TypeKey;
use std::rc::Rc;

pub mod st {
    pub type Vec3 = [f32; 3];
    pub type Vec4 = [f32; 4];
    pub type Mat4 = [Vec4; 4];
    pub type Bool = u32;
}
