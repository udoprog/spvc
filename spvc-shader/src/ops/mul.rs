use super::BadOp;
use errors::*;
use op::Op;
use reg_op::RegOp;
use shader::Shader;
use spirv::Word;
use spirv_type::SpirvType;
use std::rc::Rc;
use types::{Matrix, Vector};

/// Perform a multiply operation on the two arguments.
///
/// This operation might differ depending on the type of the arguments.
pub fn mul(lhs: Rc<Op>, rhs: Rc<Op>) -> Rc<Op> {
    if let Some(op_type) = lhs.op_type().matrix_times_matrix(rhs.op_type()) {
        return Rc::new(MatrixTimesMatrixMul {
            op_type: op_type,
            lhs: lhs,
            rhs: rhs,
        });
    }

    if let Some(op_type) = lhs.op_type().matrix_times_vector(rhs.op_type()) {
        return Rc::new(MatrixTimesVectorMul {
            op_type: op_type,
            lhs: lhs,
            rhs: rhs,
        });
    }

    Rc::new(BadOp::new("mul", "argument type mismatch", vec![lhs, rhs]))
}

#[derive(Debug)]
pub struct MatrixTimesMatrixRegMul {
    result_type: Word,
    lhs: Box<RegOp>,
    rhs: Box<RegOp>,
}

impl RegOp for MatrixTimesMatrixRegMul {
    fn op_id(&self, shader: &mut Shader) -> Result<Option<Word>> {
        let lhs = self.lhs.op_id(shader)?.ok_or(ErrorKind::NoObjectId)?;
        let rhs = self.rhs.op_id(shader)?.ok_or(ErrorKind::NoObjectId)?;

        let id = shader.builder.matrix_times_matrix(
            self.result_type,
            None,
            lhs,
            rhs,
        )?;

        Ok(Some(id))
    }
}

#[derive(Debug)]
pub struct MatrixTimesMatrixMul {
    op_type: Matrix,
    lhs: Rc<Op>,
    rhs: Rc<Op>,
}

impl Op for MatrixTimesMatrixMul {
    fn op_type(&self) -> &SpirvType {
        &self.op_type
    }

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
        let result_type = self.op_type.register_type(shader)?;

        let lhs = self.lhs.register_op(shader)?;
        let rhs = self.rhs.register_op(shader)?;

        return Ok(Box::new(MatrixTimesMatrixRegMul {
            result_type: result_type,
            lhs: lhs,
            rhs: rhs,
        }));
    }
}

#[derive(Debug)]
pub struct MatrixTimesVectorRegMul {
    result_type: Word,
    lhs: Box<RegOp>,
    rhs: Box<RegOp>,
}

impl RegOp for MatrixTimesVectorRegMul {
    fn op_id(&self, shader: &mut Shader) -> Result<Option<Word>> {
        let lhs = self.lhs.op_id(shader)?.ok_or(ErrorKind::NoObjectId)?;
        let rhs = self.rhs.op_id(shader)?.ok_or(ErrorKind::NoObjectId)?;

        let id = shader.builder.matrix_times_vector(
            self.result_type,
            None,
            lhs,
            rhs,
        )?;

        Ok(Some(id))
    }
}

#[derive(Debug)]
pub struct MatrixTimesVectorMul {
    op_type: Vector,
    lhs: Rc<Op>,
    rhs: Rc<Op>,
}

impl Op for MatrixTimesVectorMul {
    fn op_type(&self) -> &SpirvType {
        &self.op_type
    }

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
        let result_type = self.op_type.register_type(shader)?;

        let lhs = self.lhs.register_op(shader)?;
        let rhs = self.rhs.register_op(shader)?;

        return Ok(Box::new(MatrixTimesVectorRegMul {
            result_type: result_type,
            lhs: lhs,
            rhs: rhs,
        }));
    }
}
