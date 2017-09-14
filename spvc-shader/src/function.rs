use super::errors::*;
use super::op::Op;
use super::op_key::OpKey;
use super::shader::Shader;
use super::spirv::{self, Word};
use std::rc::Rc;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    ops: Vec<Rc<Op>>,
    return_op: Option<Rc<Op>>,
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

        let return_type = if let Some(ref return_op) = self.return_op {
            return_op.op_type().register_type(shader)?
        } else {
            shader.cache_op(OpKey::Void, |s| Ok(s.builder.type_void()))?
        };

        let parameter_types: Vec<Word> = vec![];

        let fn_type = shader.cache_op(
            OpKey::Function {
                return_type: return_type,
                parameter_types: parameter_types.clone(),
            },
            |s| {
                Ok(s.builder.type_function(return_type, &parameter_types))
            },
        )?;

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

        if let Some(ref return_op) = self.return_op {
            let return_op = return_op.register_op(shader)?.op_id(shader)?.ok_or(
                ErrorKind::NoOp,
            )?;

            shader.builder.ret_value(return_op)?;
        } else {
            shader.builder.ret()?;
        }

        shader.builder.end_function()?;
        Ok(id)
    }
}

/// Builder of functions.
#[derive(Debug)]
pub struct FunctionBuilder {
    name: String,
    ops: Vec<Rc<Op>>,
}

impl FunctionBuilder {
    /// Create a new function builder.
    pub fn new(name: &str) -> FunctionBuilder {
        FunctionBuilder {
            name: String::from(name),
            ops: Vec::new(),
        }
    }

    /// Add an operation to this function builder.
    pub fn op(&mut self, op: Rc<Op>) {
        self.ops.push(op);
    }

    /// Create a function that returns void
    ///
    /// All previously appended operations will be added to the created function.
    pub fn returns_void(self) -> Function {
        Function {
            name: self.name,
            ops: self.ops,
            return_op: None,
        }
    }

    /// Create a function that returns the value of the given operation.
    pub fn returns(self, return_op: Rc<Op>) -> Function {
        Function {
            name: self.name,
            ops: self.ops,
            return_op: Some(return_op),
        }
    }
}
