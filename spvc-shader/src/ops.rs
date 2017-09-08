use super::errors::*;
use super::registered_statement::RegisteredStatement;
use super::shader::Shader;
use super::spirv::Word;
use super::spirv_type::SpirvType;
use super::statement::Statement;
use std::rc::Rc;

#[derive(Debug)]
pub struct Mul<L, R> {
    lhs: L,
    rhs: R,
}

impl<L, R> Mul<L, R> {
    pub fn new(lhs: L, rhs: R) -> Mul<L, R> {
        Mul { lhs: lhs, rhs: rhs }
    }
}

#[derive(Debug)]
pub struct MatrixByMatrixMul {
    result_type: Word,
    lhs: Box<RegisteredStatement>,
    rhs: Box<RegisteredStatement>,
}

impl RegisteredStatement for MatrixByMatrixMul {
    fn statement_id(&self, shader: &mut Shader) -> Result<Word> {
        let lhs = self.lhs.statement_id(shader)?;
        let rhs = self.rhs.statement_id(shader)?;

        let id = shader.builder.matrix_times_matrix(
            self.result_type,
            None,
            lhs,
            rhs,
        )?;

        Ok(id)
    }
}

impl Statement for Mul<Rc<Box<Statement>>, Rc<Box<Statement>>> {
    fn statement_type(&self) -> &SpirvType {
        self.lhs.statement_type()
    }

    fn register_statement(&self, shader: &mut Shader) -> Result<Box<RegisteredStatement>> {
        let result_type = self.lhs.statement_type().register_type(shader)?;

        let lhs = self.lhs.register_statement(shader)?;
        let rhs = self.rhs.register_statement(shader)?;

        let lhs_matrix = self.lhs.statement_type().matrix_dims();
        let rhs_matrix = self.rhs.statement_type().matrix_dims();

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
