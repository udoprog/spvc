use errors::*;
use op::Op;
use reg_op::RegOp;
use shader::Shader;
use spirv_type::{NoType, SpirvType};
use std::collections::LinkedList;
use std::rc::Rc;

/// Reflects a bad operation.
///
/// These can be returned in cases where an operation can be statically determined not to have
/// a valid value.
#[derive(Debug)]
pub struct BadOp {
    op_type: NoType,
    op_name: &'static str,
    reason: &'static str,
    causes: Vec<Rc<Op>>,
}

impl BadOp {
    /// Create a new bad operation.
    pub fn new(op_name: &'static str, reason: &'static str, causes: Vec<Rc<Op>>) -> BadOp {
        BadOp {
            op_type: NoType,
            op_name: op_name,
            reason: reason,
            causes: causes,
        }
    }
}

impl Op for BadOp {
    fn op_type(&self) -> &SpirvType {
        &self.op_type
    }

    fn register_op(&self, _: &mut Shader) -> Result<Box<RegOp>> {
        let mut current = self;

        let mut queue: LinkedList<&BadOp> = LinkedList::new();
        queue.push_back(self);

        while let Some(next) = queue.pop_front() {
            if let Some(bad_op) = next.as_bad_op() {
                current = bad_op;

                for c in &bad_op.causes {
                    if let Some(bad_op) = c.as_bad_op() {
                        queue.push_back(bad_op);
                    }
                }
            }
        }

        let arguments: Vec<String> = current
            .causes
            .iter()
            .map(|r| r.op_type().display())
            .collect();

        Err(
            ErrorKind::BadOp(current.op_name, current.reason, arguments).into(),
        )
    }

    fn as_bad_op(&self) -> Option<&Self> {
        Some(self)
    }
}
