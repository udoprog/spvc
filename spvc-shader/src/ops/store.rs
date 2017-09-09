use errors::*;
use op::Op;
use reg_op::RegOp;
use reg_var::RegVar;
use shader::Shader;
use spirv::Word;
use spirv_type::SpirvType;
use std::rc::Rc;
use var::Var;

#[derive(Debug)]
pub struct Store {
    destination: Rc<Box<Var>>,
    source: Rc<Box<Op>>,
}

impl Store {
    pub fn new(destination: Rc<Box<Var>>, source: Rc<Box<Op>>) -> Rc<Box<Op>> {
        Rc::new(Box::new(Store {
            destination: destination,
            source: source,
        }))
    }
}

impl Op for Store {
    fn op_type(&self) -> &SpirvType {
        self.destination.var_type()
    }

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
        let result_type = self.destination.var_type().register_type(shader)?;
        let destination = self.destination.register_var(shader)?;
        let source = self.source.register_op(shader)?;

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
    destination: Box<RegVar>,
    source: Box<RegOp>,
}

impl RegOp for RegisteredStore {
    fn op_id(&self, shader: &mut Shader) -> Result<Option<Word>> {
        let pointer = self.destination.var_id(shader)?;

        let source = self.source.op_id(shader)?.ok_or(ErrorKind::NoObjectId)?;

        shader.builder.store(pointer, source, None, vec![])?;
        Ok(None)
    }
}
