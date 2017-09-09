use rspirv;

error_chain! {
    foreign_links {
        Rspirv(rspirv::mr::Error);
    }

    errors {
        BadOp(op: &'static str, reason: &'static str, arguments: Vec<String>) {
        }

        NoObjectId {
        }

        NoOp {
        }

        /// Illegal operation on NoType.
        NoType {
        }
    }
}
