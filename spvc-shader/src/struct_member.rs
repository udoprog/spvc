use super::spirv_type::SpirvType;
use std::rc::Rc;

#[derive(Debug)]
pub struct StructMember {
    pub name: &'static str,
    pub ty: Rc<SpirvType>,
    pub index: u32,
}

impl StructMember {
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
