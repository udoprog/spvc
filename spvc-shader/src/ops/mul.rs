use errors::*;
use op::Op;
use reg_op::RegOp;
use shader::Shader;
use spirv::Word;
use spirv_type::SpirvType;
use std::rc::Rc;

pub fn mul(lhs: Rc<Box<Op>>, rhs: Rc<Box<Op>>) -> Result<Rc<Box<Op>>> {
    if let (Some(lhs_dims), Some(rhs_dims)) =
        (lhs.op_type().matrix_dims(), rhs.op_type().matrix_dims())
    {
        if lhs_dims.cols != rhs_dims.rows {
            return Err(ErrorKind::MatrixMulMismatch(lhs_dims, rhs_dims).into());
        }

        return Ok(Rc::new(Box::new(MatrixByMatrixMul { lhs: lhs, rhs: rhs })));
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
    lhs: Rc<Box<Op>>,
    rhs: Rc<Box<Op>>,
}

impl Op for MatrixByMatrixMul {
    fn op_type(&self) -> &SpirvType {
        self.lhs.op_type()
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
