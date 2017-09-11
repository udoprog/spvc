use super::errors::*;
use super::matrix_dims::MatrixDims;
use super::pointer::Pointer;
use super::shader::Shader;
use super::spirv::Word;
use super::types::{Bool, Float, Matrix, Struct, UnsignedInteger, Vector};
use super::vector_dims::VectorDims;
use std::fmt;

/// Reflects a type in SPIR-V.
pub trait SpirvType: fmt::Debug {
    /// Display a representation of this type.
    fn display(&self) -> String;

    fn register_type(&self, shader: &mut Shader) -> Result<Word>;

    /// Checks if the current type is suitable for a matrix-by-matrix multiplication.
    fn matrix_times_matrix(&self, _other: &SpirvType) -> Option<Matrix> {
        None
    }

    /// Checks if the current type is suitable for a matrix-by-vector multiplication.
    fn matrix_times_vector(&self, _other: &SpirvType) -> Option<Vector> {
        None
    }

    /// Hook to register extra directives when this type is the member of a struct.
    fn register_struct_extra(&self, _id: Word, _index: u32, _shader: &mut Shader) -> Result<()> {
        Ok(())
    }

    /// Returns dimensions of this type as a matrix.
    /// None if type is not a matrix.
    fn as_matrix_dims(&self) -> Option<MatrixDims> {
        None
    }

    /// Returns dimensions of this type as a vector.
    /// None if type is not a vector.
    fn as_vector_dims(&self) -> Option<VectorDims> {
        None
    }

    /// Reflects type as pointer.
    /// None if not a pointer.
    fn as_pointer(&self) -> Option<Pointer> {
        None
    }

    fn as_float(&self) -> Option<Float> {
        None
    }

    fn as_bool(&self) -> Option<Bool> {
        None
    }

    fn as_unsigned_integer(&self) -> Option<UnsignedInteger> {
        None
    }

    fn as_vector(&self) -> Option<Vector> {
        None
    }

    fn as_matrix(&self) -> Option<Matrix> {
        None
    }

    fn as_struct(&self) -> Option<Struct> {
        None
    }

    fn as_no_type(&self) -> Option<NoType> {
        None
    }

    /// Returns dimension of vector (as part of column major matrix).
    /// None if type is not a vector.
    fn row_count(&self) -> Option<u32> {
        None
    }

    /// Width in bytes of the type.
    fn width(&self) -> u32;

    /// Check if this type matches another type.
    fn matches(&self, other: &SpirvType) -> bool;

    /// Current type as vulkano format.
    #[cfg(feature = "vulkan")]
    fn as_vulkano_format(&self) -> Option<::vulkano::format::Format> {
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NoType;

impl SpirvType for NoType {
    fn display(&self) -> String {
        format!("!")
    }

    fn register_type(&self, _: &mut Shader) -> Result<Word> {
        Err(ErrorKind::NoType.into())
    }

    fn width(&self) -> u32 {
        0
    }

    fn as_no_type(&self) -> Option<NoType> {
        Some(*self)
    }

    fn matches(&self, other: &SpirvType) -> bool {
        other.as_no_type().is_some()
    }
}

pub trait WrapperType {
    fn wrapped_type(&self) -> &SpirvType;
}

impl<T> SpirvType for T
where
    T: fmt::Debug + WrapperType,
{
    fn register_type(&self, shader: &mut Shader) -> Result<Word> {
        self.wrapped_type().register_type(shader)
    }

    fn as_pointer(&self) -> Option<Pointer> {
        self.wrapped_type().as_pointer()
    }

    fn width(&self) -> u32 {
        self.wrapped_type().width()
    }

    fn matches(&self, other: &SpirvType) -> bool {
        return self.wrapped_type().matches(other);
    }

    fn display(&self) -> String {
        self.wrapped_type().display()
    }
}
