use super::errors::*;
use super::glsl_struct_member::GlslStructMember;
use super::rspirv;
use super::shader::Shader;
use super::spirv::{Decoration, Word};
use super::spirv_type::SpirvType;
use super::type_key::TypeKey;
use std::rc::Rc;

#[derive(Debug)]
pub struct GlslStruct {
    pub name: &'static str,
    pub members: Vec<Rc<GlslStructMember>>,
}

impl SpirvType for GlslStruct {
    fn register_type(&self, shader: &mut Shader) -> Result<Word> {
        let mut field_types: Vec<Word> = Vec::new();

        for m in &self.members {
            field_types.push(m.ty.register_type(shader)?);
        }

        shader.cached_type(
            TypeKey::Struct {
                name: String::from(self.name),
                field_types: field_types.clone(),
            },
            |s| {
                let id = s.builder.type_struct(field_types.clone());

                s.name(id, self.name);

                let mut offset = 0u32;

                for (index, ref m) in self.members.iter().enumerate() {
                    let index = index as u32;

                    s.member_name(id, index, m.name);

                    s.builder.member_decorate(
                        id,
                        index,
                        Decoration::Offset,
                        vec![rspirv::mr::Operand::LiteralInt32(offset)],
                    );

                    offset += m.ty.width();
                    m.ty.register_struct_extra(id, index, s)?;
                }

                s.builder.decorate(id, Decoration::Block, vec![]);
                Ok(id)
            },
        )
    }

    fn width(&self) -> u32 {
        self.members.iter().map(|m| m.ty.width()).sum()
    }
}
