use super::errors::*;
use super::load::Load;
use super::primitives::{Matrix, Vector};
use super::registered_load::RegisteredLoad;
use super::registered_statement::RegisteredStatement;
use super::shader::Shader;
use super::spirv::Word;
use super::spirv_type::SpirvType;
use super::statement::Statement;
use std::fmt;

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
pub struct RegisteredMul {
    result_type: Word,
    lhs: Box<RegisteredLoad>,
    rhs: Box<RegisteredLoad>,
}

impl RegisteredStatement for RegisteredMul {
    fn statement_id(&self, shader: &mut Shader) -> Result<Word> {
        let lhs = self.lhs.load(shader)?;
        let rhs = self.rhs.load(shader)?;

        let id = shader.builder.matrix_times_matrix(
            self.result_type,
            None,
            lhs,
            rhs,
        )?;

        Ok(id)
    }
}

impl RegisteredLoad for RegisteredMul {
    fn load(&self, shader: &mut Shader) -> Result<Word> {
        self.statement_id(shader)
    }
}

impl<L, R, T: SpirvType + fmt::Debug> Statement for Mul<L, R>
where
    L: Load<LoadedType = Matrix<Vector<T>>>,
    R: Load<LoadedType = Matrix<Vector<T>>>,
{
    fn register_statement(&self, shader: &mut Shader) -> Result<Box<RegisteredStatement>> {
        {
            let lhs_ty = self.lhs.loaded_type();
            let rhs_ty = self.rhs.loaded_type();

            if lhs_ty.column_count != rhs_ty.column_count {
                return Err("mismatching column count".into());
            }

            if lhs_ty.column_type.component_count != rhs_ty.column_type.component_count {
                return Err("mismatching inner component count".into());
            }
        }

        let result_type = self.lhs.loaded_type().register_type(shader)?;

        let lhs = self.lhs.register_load(shader)?;
        let rhs = self.rhs.register_load(shader)?;

        Ok(Box::new(RegisteredMul {
            result_type: result_type,
            lhs: lhs,
            rhs: rhs,
        }))
    }
}

impl<L, R, T: SpirvType> Load for Mul<L, R>
where
    L: Load<LoadedType = Matrix<Vector<T>>>,
    R: Load<LoadedType = Matrix<Vector<T>>>,
{
    type LoadedType = Matrix<Vector<T>>;

    fn loaded_type(&self) -> &Self::LoadedType {
        self.lhs.loaded_type()
    }

    fn register_load(&self, shader: &mut Shader) -> Result<Box<RegisteredLoad>> {
        let result_type = self.lhs.loaded_type().register_type(shader)?;

        let lhs = self.lhs.register_load(shader)?;
        let rhs = self.rhs.register_load(shader)?;

        Ok(Box::new(RegisteredMul {
            result_type: result_type,
            lhs: lhs,
            rhs: rhs,
        }))
    }
}
