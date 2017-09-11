//! # Reflection of struct members
//!
//! These are typically generated using the glsl-struct-derive crate.

use super::spirv_type::SpirvType;
use std::rc::Rc;

/// Reflects a single struct member.
#[derive(Debug)]
pub struct StructMember {
    /// Name of the struct member.
    pub name: &'static str,
    /// Type of the struct member.
    pub ty: Rc<SpirvType>,
    /// Index of the struct member.
    pub index: u32,
}

impl StructMember {
    /// Check if this member matches another.
    pub fn matches(&self, other: &StructMember) -> bool {
        if self.name != other.name {
            return false;
        }

        if !self.ty.matches(other.ty.as_ref()) {
            return false;
        }

        if self.index != other.index {
            return false;
        }

        return true;
    }
}
