use super::types::{Float, Matrix, Vector};
use super::vector_dims::VectorDims;

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

    /// Reflects the multiplication of a matrix and a matrix.
    pub fn matrix_mul_type(&self, other: MatrixDims) -> Option<Matrix> {
        if self.cols != other.rows {
            return None;
        }

        Some(Matrix::new(Vector::new(Float, self.rows), other.cols))
    }

    /// Reflects the multiplication of a matrix and a vector.
    pub fn vector_mul_type(&self, other: VectorDims) -> Option<Vector> {
        if self.rows != other.count {
            return None;
        }

        Some(Vector::new(Float, other.count))
    }

    /// Transpose the current matrix and return the new type.
    pub fn transpose_type(&self) -> Matrix {
        Matrix::new(Vector::new(Float, self.cols), self.rows)
    }
}
