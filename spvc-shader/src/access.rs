use super::errors::*;
use super::glsl_struct_member::GlslStructMember;
use super::op::Op;
use super::pointer::Pointer;
use super::reg_op::RegOp;
use super::shader::Shader;
use super::spirv::Word;
use super::spirv_type::SpirvType;
use super::storage_class::StorageClass;
use std::rc::Rc;

pub trait AccessTrait {
    fn access_member(&self, member: GlslStructMember) -> Result<Rc<Box<Op>>>;
}

impl AccessTrait for Rc<Box<Op>> {
    fn access_member(&self, member: GlslStructMember) -> Result<Rc<Box<Op>>> {
        let base = self.base().map(Clone::clone).unwrap_or_else(
            || self.clone(),
        );

        let storage_class = self.storage_class().ok_or(ErrorKind::NoStorageClass)?;

        let mut access_chain = self.access_chain()
            .map(|slice| slice.to_vec())
            .unwrap_or_else(|| vec![]);

        access_chain.push(member.index);

        let member_type = Rc::new(member.ty);

        Ok(Rc::new(Box::new(Access {
            base: base,
            storage_class: storage_class,
            pointer_type: Pointer::new(storage_class, member_type.clone()),
            accessed_type: member_type.clone(),
            access_chain: access_chain,
        })))
    }
}

/// Accessing fields on structs.
#[derive(Debug)]
pub struct Access {
    pub base: Rc<Box<Op>>,
    pub storage_class: StorageClass,
    pub pointer_type: Pointer,
    pub accessed_type: Rc<Box<SpirvType>>,
    pub access_chain: Vec<u32>,
}

#[derive(Debug)]
pub struct RegisteredAccess {
    pub base: Rc<Box<RegOp>>,
    pub result_type: Word,
    pub pointer_type: Word,
    pub access_chain: Vec<Word>,
}

impl RegOp for RegisteredAccess {
    fn op_id(&self, shader: &mut Shader) -> Result<Option<Word>> {
        let base = self.base.op_id(shader)?.ok_or(ErrorKind::NoBase)?;

        let id = shader.builder.access_chain(
            self.pointer_type,
            None,
            base,
            self.access_chain.clone(),
        )?;

        Ok(Some(id))
    }
}

impl Op for Access {
    fn base(&self) -> Option<&Rc<Box<Op>>> {
        Some(&self.base)
    }

    fn access_chain(&self) -> Option<&[u32]> {
        Some(self.access_chain.as_ref())
    }

    fn storage_class(&self) -> Option<StorageClass> {
        Some(self.storage_class)
    }

    fn op_type(&self) -> &SpirvType {
        &self.pointer_type
    }

    fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
        let base = self.base.register_op(shader)?;

        let access_chain = {
            let mut out = Vec::new();

            for a in &self.access_chain {
                out.push(shader.constant_u32(*a)?);
            }

            out
        };

        let result_type = self.accessed_type.register_type(shader)?;
        let pointer_type = self.pointer_type.register_type(shader)?;

        Ok(Box::new(RegisteredAccess {
            base: Rc::new(base),
            result_type: result_type,
            pointer_type: pointer_type,
            access_chain: access_chain,
        }))
    }
}
