use super::spirv_type::SpirvType;

#[derive(Debug)]
pub struct GlslStructMember {
    pub name: &'static str,
    pub ty: Box<SpirvType>,
    pub index: u32,
}
