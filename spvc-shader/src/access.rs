use super::errors::*;
use super::glsl_struct_member::GlslStructMember;
use super::pointer::Pointer;
use super::registered_statement::RegisteredStatement;
use super::registered_variable::RegisteredVariable;
use super::shader::Shader;
use super::spirv::Word;
use super::spirv_type::SpirvType;
use super::statement::Statement;
use super::variable::Variable;
use std::rc::Rc;

pub trait AccessTrait {
    fn access(&self, member: GlslStructMember) -> Result<Rc<Box<Statement>>>;
}

impl AccessTrait for Rc<Box<Variable>> {
    fn access(&self, member: GlslStructMember) -> Result<Rc<Box<Statement>>> {
        let storage_class = self.storage_class().ok_or(ErrorKind::NoStorageClass)?;

        Ok(Rc::new(Box::new(Access {
            base: self.clone(),
            pointer_type: Pointer(storage_class, member.ty.clone()),
            accessed_type: member.ty.clone(),
            index: member.index,
        })))
    }
}

/// Accessing fields on structs.
#[derive(Debug)]
pub struct Access {
    pub base: Rc<Box<Variable>>,
    pub pointer_type: Pointer,
    pub accessed_type: Rc<SpirvType>,
    pub index: u32,
}

#[derive(Debug)]
pub struct RegisteredAccess {
    pub base: Box<RegisteredVariable>,
    pub result_type: Word,
    pub pointer_type: Word,
    pub index_const: Word,
}

impl RegisteredStatement for RegisteredAccess {
    fn statement_id(&self, shader: &mut Shader) -> Result<Word> {
        let base = self.base.variable_id(shader)?;

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

impl Statement for Access {
    fn statement_type(&self) -> &SpirvType {
        self.accessed_type.as_ref()
    }

    fn register_statement(&self, shader: &mut Shader) -> Result<Box<RegisteredStatement>> {
        let base = self.base.register_variable(shader)?;
        let index_const = shader.constant_u32(self.index)?;
        let result_type = self.accessed_type.register_type(shader)?;
        let pointer_type = self.pointer_type.register_type(shader)?;

        Ok(Box::new(RegisteredAccess {
            base: base,
            result_type: result_type,
            pointer_type: pointer_type,
            index_const: index_const,
        }))
    }
}
