use errors::*;
use op::Op;
use primitives::Matrix;
use reg_op::RegOp;
use shader::Shader;
use spirv::Word;
use spirv_type::SpirvType;
use std::rc::Rc;

pub fn mul(lhs: Rc<Box<Op>>, rhs: Rc<Box<Op>>) -> Result<Rc<Box<Op>>> {
    if let Some(op_type) = lhs.op_type().matrix_times_matrix(rhs.op_type())? {
        return Ok(Rc::new(Box::new(MatrixByMatrixMul {
            op_type: op_type,
            lhs: lhs,
            rhs: rhs,
        })));
    }

    Err(ErrorKind::MulMismatch.into())
}

#[derive(Debug)]
pub struct MatrixByMatrixRegMul {
    result_type: Word,
    lhs: Box<RegOp>,
    rhs: Box<RegOp>,
}

impl RegOp for MatrixByMatrixRegMul {
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
pub struct MatrixByMatrixMul {
    op_type: Matrix,
    lhs: Rc<Box<Op>>,
    rhs: Rc<Box<Op>>,
}

impl Op for MatrixByMatrixMul {
    fn op_type(&self) -> &SpirvType {
        &self.op_type
    }

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
        let result_type = self.lhs.op_type().register_type(shader)?;

        let lhs = self.lhs.register_op(shader)?;
        let rhs = self.rhs.register_op(shader)?;

        return Ok(Box::new(MatrixByMatrixRegMul {
            result_type: result_type,
            lhs: lhs,
            rhs: rhs,
        }));
    }
}
