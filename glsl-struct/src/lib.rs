#[derive(PartialEq, Eq, Hash, Debug)]
pub struct GlslStruct {
    pub name: &'static str,
    pub members: Vec<GlslStructMember>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct GlslStructMember {
    pub name: &'static str,
    pub ty: GlslType,
    pub index: u32,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum GlslType {
    Bool,
    Mat4,
    Vec2,
    Vec3,
    Vec4,
}

pub type Vec4 = [f32; 4];
pub type Mat4 = [Vec4; 4];
pub type Bool = u32;
