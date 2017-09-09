use super::errors::*;
use super::primitives::{Float, Matrix, Vector};

/// Reflects the dimensions of a matrix, and how various operations affect them.
#[derive(Debug, Clone, Copy)]
pub struct MatrixDims {
    cols: u32,
    rows: u32,
}

impl MatrixDims {
    pub fn new(cols: u32, rows: u32) -> MatrixDims {
        MatrixDims {
            cols: cols,
            rows: rows,
        }
    }

    /// Multiply the current matrix, and return the new type.
    pub fn mul_type(&self, other: MatrixDims) -> Result<Matrix> {
        if self.cols != other.rows {
            return Err(ErrorKind::MatrixMulMismatch(*self, other).into());
        }

        Ok(Matrix::new(Vector::new(Float, self.rows), other.cols))
    }

    /// Transpose the current matrix and return the new type.
    pub fn transpose_type(&self) -> Result<Matrix> {
        Ok(Matrix::new(Vector::new(Float, self.cols), self.rows))
    }
}
