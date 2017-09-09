use errors::*;
use op::Op;
use reg_op::RegOp;
use shader::Shader;
use spirv::Word;
use spirv_type::SpirvType;
use std::rc::Rc;

#[derive(Debug)]
pub struct Mul {
    lhs: Rc<Box<Op>>,
    rhs: Rc<Box<Op>>,
}

impl Mul {
    pub fn new(lhs: Rc<Box<Op>>, rhs: Rc<Box<Op>>) -> Rc<Box<Op>> {
        Rc::new(Box::new(Mul { lhs: lhs, rhs: rhs }))
    }
}

#[derive(Debug)]
pub struct MatrixByMatrixMul {
    result_type: Word,
    lhs: Box<RegOp>,
    rhs: Box<RegOp>,
}

impl RegOp for MatrixByMatrixMul {
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

impl Op for Mul {
    fn op_type(&self) -> &SpirvType {
        self.lhs.op_type()
    }

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
        let result_type = self.lhs.op_type().register_type(shader)?;

        let lhs = self.lhs.register_op(shader)?;
        let rhs = self.rhs.register_op(shader)?;

        let lhs_matrix = self.lhs.op_type().matrix_dims();
        let rhs_matrix = self.rhs.op_type().matrix_dims();

        if lhs_matrix.is_some() && rhs_matrix.is_some() {
            return Ok(Box::new(MatrixByMatrixMul {
                result_type: result_type,
                lhs: lhs,
                rhs: rhs,
            }));
        }

        Err(
            format!(
                "unsupported arguments (lhs: {:?}, rhs: {:?})",
                self.lhs,
                self.rhs
            ).into(),
        )
    }
}
