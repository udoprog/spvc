use super::BadOp;
use errors::*;
use op::Op;
use pointer::Pointer;
use reg_op::RegOp;
use shader::Shader;
use spirv::Word;
use spirv_type::SpirvType;
use std::rc::Rc;

#[derive(Debug)]
pub struct Load {
    /// Pointe type of the object being loaded.
    pub pointer: Pointer,
    /// Object being loaded.
    pub object: Rc<Op>,
}

pub fn load(object: Rc<Op>) -> Rc<Op> {
    if let Some(pointer) = object.op_type().as_pointer() {
        return Rc::new(Load {
            pointer: pointer,
            object: object,
        });
    }

    Rc::new(BadOp::new("load", "expected pointer", vec![object]))
}

impl Op for Load {
    fn op_type(&self) -> &SpirvType {
        self.pointer.pointee_type.as_ref()
    }

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
        let result_type = self.pointer.pointee_type.register_type(shader)?;
        let object = self.object.register_op(shader)?;

        Ok(Box::new(RegisteredLoad {
            result_type: result_type,
            object: object,
        }))
    }
}

#[derive(Debug)]
pub struct RegisteredLoad {
    result_type: Word,
    object: Box<RegOp>,
}

impl RegOp for RegisteredLoad {
    fn op_id(&self, shader: &mut Shader) -> Result<Option<Word>> {
        let pointer = self.object.op_id(shader)?.ok_or(ErrorKind::NoOp)?;

        let id = shader.builder.load(
            self.result_type,
            None,
            pointer,
            None,
            &[],
        )?;

        Ok(Some(id))
    }
}
