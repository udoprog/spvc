use errors::*;
use op::Op;
use primitives::{Float, Vector};
use reg_op::RegOp;
use shader::Shader;
use spirv::Word;
use spirv_type::SpirvType;
use std::rc::Rc;

macro_rules! expand_vec {
    ($st:ident, $fn:ident, $orig_size:expr, $dest_size:expr, $($const:ident),*) => {
    #[derive(Debug)]
    pub struct $st {
        result_type: Vector,
        source: Rc<Box<Op>>,
        $($const: f32,)*
    }

    pub fn $fn(source: Rc<Box<Op>>, $($const: f32,)*) -> Result<Rc<Box<Op>>> {
        if let Some(vector) = source.op_type().as_vector() {
            if vector.component_count == $orig_size {
                let result_type = Vector::new(Float, $dest_size);

                return Ok(Rc::new(Box::new($st {
                    result_type: result_type,
                    source: source,
                    $($const: $const,)*
                })));
            }
        }

        Err(
            ErrorKind::VecMismatch(stringify!($fn), source.op_type().display()).into(),
        )
    }

    impl Op for $st {
        fn op_type(&self) -> &SpirvType {
            &self.result_type
        }

        fn register_op(&self, shader: &mut Shader) -> Result<Box<RegOp>> {
            let component_type = self.result_type.component.register_type(shader)?;
            let result_type = self.result_type.register_type(shader)?;
            let source = self.source.register_op(shader)?;

            $(
            let $const = shader.constant_f32(self.$const)?;
            )*

            Ok(Box::new(VecExpand {
                component_type: component_type,
                result_type: result_type,
                source: source,
                size: $orig_size,
                constants: vec![$($const),*],
            }))
        }
    }
    };
}

expand_vec!(Vec2ToVec3, vec2_to_vec3, 2, 3, z);
expand_vec!(Vec2ToVec4, vec2_to_vec4, 2, 4, z, w);

expand_vec!(Vec3ToVec4, vec3_to_vec4, 3, 4, z);

/// Expand a vector.
#[derive(Debug)]
pub struct VecExpand {
    /// The extract component type.
    component_type: Word,
    /// The type of the resulting object.
    result_type: Word,
    /// Source of the vector to expand.
    source: Box<RegOp>,
    /// Previous size of the vector.
    size: u32,
    /// Constant values to expand the vector with.
    constants: Vec<Word>,
}

impl RegOp for VecExpand {
    fn op_id(&self, shader: &mut Shader) -> Result<Option<Word>> {
        let composite = self.source.op_id(shader)?.ok_or(ErrorKind::NoOp)?;

        let mut constituents = Vec::new();

        for i in 0u32..self.size {
            let id = shader.builder.composite_extract(
                self.component_type,
                None,
                composite,
                vec![i],
            )?;

            constituents.push(id);
        }

        // expand with specified constants
        constituents.extend(self.constants.iter().cloned());

        let id = shader.builder.composite_construct(
            self.result_type,
            None,
            constituents,
        )?;

        Ok(Some(id))
    }
}
