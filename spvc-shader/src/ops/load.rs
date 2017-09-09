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
pub struct Load {
    variable: Rc<Box<Variable>>,
}

impl Load {
    pub fn new(variable: Rc<Box<Variable>>) -> Rc<Box<Statement>> {
        Rc::new(Box::new(Load { variable: variable }))
    }
}

impl Statement for Load {
    fn statement_type(&self) -> &SpirvType {
        self.variable.variable_type()
    }

    fn register_statement(&self, shader: &mut Shader) -> Result<Box<RegisteredStatement>> {
        let result_type = self.variable.variable_type().register_type(shader)?;
        let variable = self.variable.register_variable(shader)?;

        Ok(Box::new(RegisteredLoad {
            result_type: result_type,
            variable: Rc::new(variable),
        }))
    }
}

#[derive(Debug)]
pub struct RegisteredLoad {
    result_type: Word,
    variable: Rc<Box<RegisteredVariable>>,
}

impl RegisteredStatement for RegisteredLoad {
    fn statement_id(&self, shader: &mut Shader) -> Result<Word> {
        let pointer = self.variable.variable_id(shader)?;

        let id = shader.builder.load(
            self.result_type,
            None,
            pointer,
            None,
            vec![],
        )?;

        Ok(id)
    }
}
