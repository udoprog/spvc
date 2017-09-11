//! Errors for SPVC

use rspirv;

error_chain! {
    foreign_links {
        Rspirv(rspirv::mr::Error);
        VulkanoOomError(::vulkano::OomError) #[cfg(feature = "vulkan")];
        FfiNulError(::std::ffi::NulError);
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

        /// Operation was not an interface.
        NotInterface {
        }

        IllegalInterfaceType {
        }
    }
}
