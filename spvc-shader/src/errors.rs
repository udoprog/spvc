use super::MatrixDims;
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

        MulMismatch {
        }

        MatrixMulMismatch(lhs: MatrixDims, rhs: MatrixDims) {
        }
    }
}
