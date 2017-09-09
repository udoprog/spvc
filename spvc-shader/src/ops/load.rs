use errors::*;
use op::Op;
use reg_op::RegOp;
use shader::Shader;
use spirv::Word;
use spirv_type::SpirvType;
use std::rc::Rc;

#[derive(Debug)]
pub struct Load {
    var: Rc<Box<Op>>,
}

impl Load {
    pub fn new(var: Rc<Box<Op>>) -> Rc<Box<Op>> {
        Rc::new(Box::new(Load { var: var }))
    }
}

impl Op for Load {
    fn op_type(&self) -> &SpirvType {
        self.var.op_type()
    }

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
        let result_type = self.var.op_type().register_type(shader)?;
        let var = self.var.register_op(shader)?;

        Ok(Box::new(RegisteredLoad {
            result_type: result_type,
            var: Rc::new(var),
        }))
    }
}

#[derive(Debug)]
pub struct RegisteredLoad {
    result_type: Word,
    var: Rc<Box<RegOp>>,
}

impl RegOp for RegisteredLoad {
    fn op_id(&self, shader: &mut Shader) -> Result<Option<Word>> {
        let pointer = self.var.op_id(shader)?.ok_or(ErrorKind::NoOp)?;

        let id = shader.builder.load(
            self.result_type,
            None,
            pointer,
            None,
            vec![],
        )?;

        Ok(Some(id))
    }
}
