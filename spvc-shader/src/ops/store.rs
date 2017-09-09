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
    dest: Rc<Box<Op>>,
    dest_type: Pointer,
    source: Rc<Box<Op>>,
}

pub fn store(dest: Rc<Box<Op>>, source: Rc<Box<Op>>) -> Result<Rc<Box<Op>>> {
    let dest_type = dest.op_type().as_pointer().ok_or(
        ErrorKind::ExpectedPointer(
            "store",
            source.op_type().display(),
        ),
    )?;

    if !dest_type.pointee_type.matches(source.op_type()) {
        return Err(
            ErrorKind::StoreMismatch("store", dest_type.display(), source.op_type().display())
                .into(),
        );
    }

    Ok(Rc::new(Box::new(Store {
        dest: dest,
        dest_type: dest_type,
        source: source,
    })))
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

        shader.builder.store(pointer, source, None, vec![])?;
        Ok(None)
    }
}
