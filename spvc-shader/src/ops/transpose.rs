use errors::*;
use op::Op;
use reg_op::RegOp;
use shader::Shader;
use spirv::Word;
use spirv_type::SpirvType;
use std::rc::Rc;

#[derive(Debug)]
pub struct Transpose {
    matrix: Rc<Box<Op>>,
}

impl Transpose {
    pub fn new(matrix: Rc<Box<Op>>) -> Rc<Box<Op>> {
        Rc::new(Box::new(Transpose { matrix: matrix }))
    }
}

impl Op for Transpose {
    fn op_type(&self) -> &SpirvType {
        self.matrix.op_type()
    }

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
        let op_type = self.matrix.op_type();

        if op_type.matrix_dims().is_none() {
            return Err(ErrorKind::BadArgument.into());
        }

        let result_type = op_type.register_type(shader)?;
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
