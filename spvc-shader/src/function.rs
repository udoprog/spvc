use super::errors::*;
use super::shader::Shader;
use super::spirv::{self, Word};
use super::spirv_type::SpirvType;
use super::statement::Statement;
use super::type_key::TypeKey;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    statements: Vec<Box<Statement>>,
    returns: Option<Box<SpirvType>>,
}

impl Function {
    pub fn register_function(self, shader: &mut Shader) -> Result<Word> {
        let statements = self.statements;

        let statements = {
            let mut out = Vec::new();

            for s in statements {
                out.push(s.register_statement(shader)?);
            }

            out
        };

        let return_type = self.returns
            .as_ref()
            .map(|r| r.register_type(shader))
            .unwrap_or_else(|| {
                shader.cached_type(TypeKey::Void, |s| Ok(s.builder.type_void()))
            })?;

        let parameter_types: Vec<Word> = vec![];

        let fn_key = TypeKey::Function {
            return_type: return_type,
            parameter_types: parameter_types.clone(),
        };

        let fn_type = shader.cached_type(fn_key, |s| {
            Ok(s.builder.type_function(
                return_type,
                parameter_types.clone(),
            ))
        })?;

        let id = shader.builder.begin_function(
            return_type,
            None,
            spirv::FUNCTION_CONTROL_NONE,
            fn_type,
        )?;

        let _label_start_fn = shader.builder.begin_basic_block(None)?;

        for s in statements {
            s.statement_id(shader)?;
        }

        shader.builder.ret()?;
        shader.builder.end_function()?;
        Ok(id)
    }
}

#[derive(Debug)]
pub struct FunctionBuilder {
    name: String,
    statements: Vec<Box<Statement>>,
}

impl FunctionBuilder {
    pub fn new(name: &str) -> FunctionBuilder {
        FunctionBuilder {
            name: String::from(name),
            statements: Vec::new(),
        }
    }

    pub fn statement<S: 'static + Statement>(&mut self, statement: S) {
        self.statements.push(Box::new(statement));
    }

    pub fn returns_void(self) -> Function {
        Function {
            name: self.name,
            statements: self.statements,
            returns: None,
        }
    }

    pub fn returns<T: 'static + SpirvType>(self, ty: T) -> Function {
        Function {
            name: self.name,
            statements: self.statements,
            returns: Some(Box::new(ty)),
        }
    }
}
