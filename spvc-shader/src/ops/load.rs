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
pub struct Load {
    var: Rc<Box<Var>>,
}

impl Load {
    pub fn new(var: Rc<Box<Var>>) -> Rc<Box<Op>> {
        Rc::new(Box::new(Load { var: var }))
    }
}

impl Op for Load {
    fn op_type(&self) -> &SpirvType {
        self.var.var_type()
    }

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
        let result_type = self.var.var_type().register_type(shader)?;
        let var = self.var.register_var(shader)?;

        Ok(Box::new(RegisteredLoad {
            result_type: result_type,
            var: Rc::new(var),
        }))
    }
}

#[derive(Debug)]
pub struct RegisteredLoad {
    result_type: Word,
    var: Rc<Box<RegVar>>,
}

impl RegOp for RegisteredLoad {
    fn op_id(&self, shader: &mut Shader) -> Result<Option<Word>> {
        let pointer = self.var.var_id(shader)?;

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
