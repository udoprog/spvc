use super::matrix_dims::MatrixDims;
use super::vector_dims::VectorDims;
use rspirv;

error_chain! {
    foreign_links {
        Rspirv(rspirv::mr::Error);
    }

    errors {
        MissingType {
        }

        NoBase {
        }

        NoOp {
        }

        NoStorageClass {
        }

        NoObjectId {
        }

        /// Bad argument to an operation.
        BadArgument {
        }

        MatrixTimesMatrixMismatch(lhs: MatrixDims, rhs: MatrixDims) {
        }

        MatrixTimesVectorMismatch(lhs: MatrixDims, rhs: VectorDims) {
        }

        ExpectedMatrix(op: &'static str, actual: String) {
        }

        ExpectedPointer(op: &'static str, actual: String) {
        }

        ArgumentMismatch(op: &'static str, arguments: Vec<String>) {
        }

        StoreMismatch(op: &'static str, dest: String, source: String) {
        }

        VecMismatch(op: &'static str, source: String) {
        }
    }
}
