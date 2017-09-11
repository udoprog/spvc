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
pub struct Store {
    dest: Rc<Op>,
    dest_type: Pointer,
    source: Rc<Op>,
}

pub fn store(dest: Rc<Op>, source: Rc<Op>) -> Rc<Op> {
    if let Some(dest_type) = dest.op_type().as_pointer() {
        if dest_type.pointee_type.matches(source.op_type()) {
            return Rc::new(Store {
                dest: dest,
                dest_type: dest_type,
                source: source,
            });
        }
    }

    Rc::new(BadOp::new(
        "store",
        "argument type mismatch",
        vec![dest, source],
    ))
}

impl Op for Store {
    fn op_type(&self) -> &SpirvType {
        self.dest.op_type()
    }

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
        let result_type = self.dest.op_type().register_type(shader)?;
        let dest = self.dest.register_op(shader)?;
        let source = self.source.register_op(shader)?;

        Ok(Box::new(RegisteredStore {
            result_type: result_type,
            dest: dest,
            source: source,
        }))
    }
}

#[derive(Debug)]
pub struct RegisteredStore {
    result_type: Word,
    dest: Box<RegOp>,
    source: Box<RegOp>,
}

impl RegOp for RegisteredStore {
    fn op_id(&self, shader: &mut Shader) -> Result<Option<Word>> {
        let pointer = self.dest.op_id(shader)?.ok_or(ErrorKind::NoOp)?;
        let source = self.source.op_id(shader)?.ok_or(ErrorKind::NoObjectId)?;

        shader.builder.store(pointer, source, None, &[])?;
        Ok(None)
    }
}
