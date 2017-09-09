mod load;
mod store;
mod mul;
mod transpose;
mod expand_vec;
mod bad_op;

pub use self::bad_op::BadOp;
pub use self::expand_vec::*;
pub use self::load::load;
pub use self::mul::mul;
pub use self::store::store;
pub use self::transpose::transpose;
