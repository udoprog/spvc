use super::spirv_type::SpirvType;

#[derive(Debug)]
pub struct GlslStructMember {
    pub name: &'static str,
    pub ty: Box<SpirvType>,
    pub index: u32,
}

impl GlslStructMember {
    pub fn matches(&self, other: &GlslStructMember) -> bool {
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
