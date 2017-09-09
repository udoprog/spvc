use errors::*;
use op::Op;
use primitives::Matrix;
use reg_op::RegOp;
use shader::Shader;
use spirv::Word;
use spirv_type::SpirvType;
use std::rc::Rc;

#[derive(Debug)]
pub struct Transpose {
    op_type: Matrix,
    matrix: Rc<Box<Op>>,
}

/// Reflects a transpose operation.
pub fn transpose(matrix: Rc<Box<Op>>) -> Result<Rc<Box<Op>>> {
    // Expect a transposable matrix as argument type.
    let op_type = matrix
        .op_type()
        .as_matrix_dims()
        .ok_or(ErrorKind::ExpectedMatrix(
            "transpose",
            matrix.op_type().display(),
        ))?
        .transpose_type()?;

    Ok(Rc::new(Box::new(Transpose {
        op_type: op_type,
        matrix: matrix,
    })))
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
