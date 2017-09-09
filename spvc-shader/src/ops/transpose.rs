use super::BadOp;
use errors::*;
use op::Op;
use reg_op::RegOp;
use shader::Shader;
use spirv::Word;
use spirv_type::SpirvType;
use std::rc::Rc;
use types::Matrix;

#[derive(Debug)]
pub struct Transpose {
    op_type: Matrix,
    matrix: Rc<Box<Op>>,
}

/// Reflects a transpose operation.
pub fn transpose(matrix: Rc<Box<Op>>) -> Rc<Box<Op>> {
    // Expect a matrix as argument type.
    if let Some(dims) = matrix.op_type().as_matrix_dims() {
        let op_type = dims.transpose_type();

        return Rc::new(Box::new(Transpose {
            op_type: op_type,
            matrix: matrix,
        }));
    }

    Rc::new(Box::new(BadOp::new(
        "transpose",
        "expected transposable matrix",
        vec![matrix],
    )))
}

impl Op for Transpose {
    fn op_type(&self) -> &SpirvType {
        &self.op_type
    }

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
        let result_type = self.op_type.register_type(shader)?;
        let matrix = self.matrix.register_op(shader)?;

        Ok(Box::new(RegTranspose {
            result_type: result_type,
            matrix: Rc::new(matrix),
        }))
    }
}

#[derive(Debug)]
pub struct RegTranspose {
    result_type: Word,
    matrix: Rc<Box<RegOp>>,
}

impl RegOp for RegTranspose {
    fn op_id(&self, shader: &mut Shader) -> Result<Option<Word>> {
        let matrix = self.matrix.op_id(shader)?.ok_or(ErrorKind::NoOp)?;
        let id = shader.builder.transpose(self.result_type, None, matrix)?;
        Ok(Some(id))
    }
}
