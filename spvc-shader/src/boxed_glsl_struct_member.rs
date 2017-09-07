use super::spirv_type::SpirvType;
use std::marker;
use std::rc::Rc;

/// A struct member with a boxed type.
#[derive(Debug)]
pub struct BoxedGlslStructMember<S> {
    pub name: &'static str,
    pub ty: Rc<SpirvType>,
    pub index: u32,
    pub struct_marker: marker::PhantomData<S>,
}
