use super::spirv_type::SpirvType;
use std::rc::Rc;

#[derive(Debug)]
pub struct GlslStructMember {
    pub name: &'static str,
    pub ty: Rc<SpirvType>,
    pub index: u32,
}
