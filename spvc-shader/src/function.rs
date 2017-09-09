use super::errors::*;
use super::op::Op;
use super::shader::Shader;
use super::spirv::{self, Word};
use super::spirv_type::SpirvType;
use super::type_key::TypeKey;
use std::rc::Rc;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    ops: Vec<Rc<Box<Op>>>,
    returns: Option<Box<SpirvType>>,
}

impl Function {
    pub fn register_function(self, shader: &mut Shader) -> Result<Word> {
        let ops = self.ops;

        let ops = {
            let mut out = Vec::new();

            for s in ops {
                out.push(s.register_op(shader)?);
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

        for s in ops {
            s.op_id(shader)?;
        }

        shader.builder.ret()?;
        shader.builder.end_function()?;
        Ok(id)
    }
}

#[derive(Debug)]
pub struct FunctionBuilder {
    name: String,
    ops: Vec<Rc<Box<Op>>>,
}

impl FunctionBuilder {
    pub fn new(name: &str) -> FunctionBuilder {
        FunctionBuilder {
            name: String::from(name),
            ops: Vec::new(),
        }
    }

    pub fn op(&mut self, op: Rc<Box<Op>>) {
        self.ops.push(op);
    }

    pub fn returns_void(self) -> Function {
        Function {
            name: self.name,
            ops: self.ops,
            returns: None,
        }
    }

    pub fn returns<T: 'static + SpirvType>(self, ty: T) -> Function {
        Function {
            name: self.name,
            ops: self.ops,
            returns: Some(Box::new(ty)),
        }
    }
}
