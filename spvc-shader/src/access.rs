use super::errors::*;
use super::load::Load;
use super::pointer::Pointer;
use super::registered_load::RegisteredLoad;
use super::registered_variable::RegisteredVariable;
use super::shader::Shader;
use super::spirv::Word;
use super::spirv_type::SpirvType;
use super::variable::Variable;
use std::rc::Rc;

/// Accessing fields on structs.
#[derive(Debug)]
pub struct Access<B, M> {
    pub base: B,
    pub pointer_type: Pointer<M>,
    pub loaded_type: Rc<M>,
    pub index: u32,
}

#[derive(Debug)]
pub struct RegisteredAccess {
    pub base: Box<RegisteredVariable>,
    pub result_type: Word,
    pub pointer_type: Word,
    pub index_const: Word,
}

impl RegisteredLoad for RegisteredAccess {
    fn load(&self, shader: &mut Shader) -> Result<Word> {
        let base = self.base.variable_id()?;

        let access_id = shader.builder.access_chain(
            self.pointer_type,
            None,
            base,
            vec![self.index_const],
        )?;

        let id = shader.builder.load(
            self.result_type,
            None,
            access_id,
            None,
            vec![],
        )?;

        Ok(id)
    }
}

impl<B: Variable, M: SpirvType> Load for Access<B, M> {
    type LoadedType = M;

    fn loaded_type(&self) -> &Self::LoadedType {
        self.loaded_type.as_ref()
    }

    fn register_load(&self, shader: &mut Shader) -> Result<Box<RegisteredLoad>> {
        let base = self.base.register_variable(shader)?;
        let index_const = shader.constant_u32(self.index)?;
        let result_type = self.loaded_type.register_type(shader)?;
        let pointer_type = self.pointer_type.register_type(shader)?;

        Ok(Box::new(RegisteredAccess {
            base: base,
            result_type: result_type,
            pointer_type: pointer_type,
            index_const: index_const,
        }))
    }
}
