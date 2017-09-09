use super::errors::*;
use super::matrix_dims::MatrixDims;
use super::primitives::Matrix;
use super::shader::Shader;
use super::spirv::Word;
use std::fmt;

/// Reflects a type in SPIR-V.
pub trait SpirvType: fmt::Debug {
    /// Display a representation of this type.
    fn display(&self) -> String {
        format!("{:?}", self)
    }

    fn register_type(&self, shader: &mut Shader) -> Result<Word>;

    /// Checks if the current type is suitable for a matrix-by-matrix multiplication.
    fn matrix_times_matrix(&self, _other: &SpirvType) -> Result<Option<Matrix>> {
        Ok(None)
    }

    /// Hook to register extra directives when this type is the member of a struct.
    fn register_struct_extra(&self, _id: Word, _index: u32, _shader: &mut Shader) -> Result<()> {
        Ok(())
    }

    /// Returns dimension of this type as a matrix.
    /// None if type is not a matrix.
    fn as_matrix_dims(&self) -> Option<MatrixDims> {
        None
    }

    /// Returns dimension of vector (as part of column major matrix).
    /// None if type is not a vector.
    fn row_count(&self) -> Option<u32> {
        None
    }

    /// Width in bytes of the type.
    fn width(&self) -> u32;
}
