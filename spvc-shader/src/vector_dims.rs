/// Reflects the dimensions of a matrix, and how various operations affect them.
#[derive(Debug, Clone, Copy)]
pub struct VectorDims {
    pub count: u32,
}

impl VectorDims {
    pub fn new(count: u32) -> VectorDims {
        VectorDims { count: count }
    }
}
