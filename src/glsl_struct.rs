pub struct GlslStruct {
    pub name: String,
    pub members: Vec<GlslStructMember>,
}

pub struct GlslStructMember {
    pub name: String,
    pub ty: GlslType,
}

pub enum GlslType {
    Bool,
    Mat4,
    Vec2,
    Vec3,
    Vec4,
}

pub trait GlslStructInfo {
    fn glsl_struct() -> GlslStruct;
}
