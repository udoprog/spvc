use super::boxed_glsl_struct_member::BoxedGlslStructMember;
use super::spirv_type::SpirvType;
use std::marker;
use std::rc::Rc;

#[derive(Debug)]
pub struct GlslStructMember<S, T> {
    pub name: &'static str,
    pub ty: Rc<T>,
    pub index: u32,
    pub struct_marker: marker::PhantomData<S>,
}

impl<S, T: 'static + SpirvType> GlslStructMember<S, T> {
    pub fn boxed(self) -> BoxedGlslStructMember<S> {
        BoxedGlslStructMember {
            name: self.name,
            ty: self.ty.clone(),
            index: self.index,
            struct_marker: self.struct_marker,
        }
    }
}
