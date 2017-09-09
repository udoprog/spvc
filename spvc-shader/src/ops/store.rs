use errors::*;
use registered_statement::RegisteredStatement;
use registered_variable::RegisteredVariable;
use shader::Shader;
use spirv::Word;
use spirv_type::SpirvType;
use statement::Statement;
use std::rc::Rc;
use variable::Variable;

#[derive(Debug)]
pub struct Store {
    destination: Rc<Box<Variable>>,
    source: Rc<Box<Statement>>,
}

impl Store {
    pub fn new(destination: Rc<Box<Variable>>, source: Rc<Box<Statement>>) -> Rc<Box<Statement>> {
        Rc::new(Box::new(Store {
            destination: destination,
            source: source,
        }))
    }
}

impl Statement for Store {
    fn statement_type(&self) -> &SpirvType {
        self.destination.variable_type()
    }

    fn register_statement(&self, shader: &mut Shader) -> Result<Box<RegisteredStatement>> {
        let result_type = self.destination.variable_type().register_type(shader)?;
        let destination = self.destination.register_variable(shader)?;
        let source = self.source.register_statement(shader)?;

        Ok(Box::new(RegisteredStore {
            result_type: result_type,
            destination: destination,
            source: source,
        }))
    }
}

#[derive(Debug)]
pub struct RegisteredStore {
    result_type: Word,
    destination: Box<RegisteredVariable>,
    source: Box<RegisteredStatement>,
}

impl RegisteredStatement for RegisteredStore {
    fn statement_id(&self, shader: &mut Shader) -> Result<Option<Word>> {
        let pointer = self.destination.variable_id(shader)?;

        let source = self.source.statement_id(shader)?.ok_or(
            ErrorKind::NoObjectId,
        )?;

        shader.builder.store(pointer, source, None, vec![])?;
        Ok(None)
    }
}
