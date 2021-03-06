use super::errors::*;
use super::matrix_dims::MatrixDims;
use super::op_key::OpKey;
use super::rspirv::mr::Operand;
use super::shader::Shader;
use super::spirv::{Decoration, Word};
use super::spirv_type::SpirvType;
use super::struct_member::StructMember;
use super::vector_dims::VectorDims;
use std::rc::Rc;

/// Types which are used as fields for automatic structs.
pub mod st {
    /// The GLSL type vec2 in Rust.
    pub type Vec2 = [f32; 2];
    /// The GLSL type vec3 in Rust.
    pub type Vec3 = [f32; 3];
    /// The GLSL type vec4 in Rust.
    pub type Vec4 = [f32; 4];
    /// The GLSL type mat3 in Rust.
    pub type Mat3 = [Vec3; 3];
    /// The GLSL type mat4 in Rust.
    pub type Mat4 = [Vec4; 4];
    /// The GLSL type float in Rust.
    pub type Float = f32;
    /// The GLSL type bool in Rust.
    pub type Bool = u32;
}

/// Corresponds to the GLSL type vec2.
pub fn vec2() -> Vector {
    Vector::new(Float, 3)
}

/// Corresponds to the GLSL type vec3.
pub fn vec3() -> Vector {
    Vector::new(Float, 3)
}

/// Corresponds to the GLSL type vec4.
pub fn vec4() -> Vector {
    Vector::new(Float, 4)
}

/// Corresponds to the GLSL type mat3.
pub fn mat3() -> Matrix {
    Matrix::new(Vector::new(Float, 3), 3)
}

/// Corresponds to the GLSL type mat4.
pub fn mat4() -> Matrix {
    Matrix::new(Vector::new(Float, 4), 4)
}

/// Reflects a 32-bit floating point value.
#[derive(Debug, Clone, Copy)]
pub struct Float;

impl SpirvType for Float {
    fn register_type(&self, shader: &mut Shader) -> Result<Word> {
        shader.cache_op(OpKey::Float { width: 32 }, |s| Ok(s.builder.type_float(32)))
    }

    fn width(&self) -> u32 {
        4
    }

    fn matches(&self, other: &SpirvType) -> bool {
        other.as_float().is_some()
    }

    fn as_float(&self) -> Option<Float> {
        Some(*self)
    }

    fn display(&self) -> String {
        String::from("float")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UnsignedInteger;

impl SpirvType for UnsignedInteger {
    fn register_type(&self, shader: &mut Shader) -> Result<Word> {
        shader.cache_op(OpKey::UnsignedInteger { width: 32 }, |s| {
            Ok(s.builder.type_int(32, 0))
        })
    }

    fn width(&self) -> u32 {
        4
    }

    fn matches(&self, other: &SpirvType) -> bool {
        other.as_unsigned_integer().is_some()
    }

    fn as_unsigned_integer(&self) -> Option<UnsignedInteger> {
        Some(*self)
    }

    fn display(&self) -> String {
        String::from("uint32_t")
    }
}

/// Reflects a boolean value.
#[derive(Debug, Clone, Copy)]
pub struct Bool;

impl SpirvType for Bool {
    fn register_type(&self, shader: &mut Shader) -> Result<Word> {
        shader.cache_op(OpKey::Bool, |s| Ok(s.builder.type_bool()))
    }

    fn width(&self) -> u32 {
        4
    }

    fn matches(&self, other: &SpirvType) -> bool {
        other.as_bool().is_some()
    }

    fn as_bool(&self) -> Option<Bool> {
        Some(*self)
    }

    fn display(&self) -> String {
        String::from("bool")
    }
}

/// Reflects a vector of known size, an uniform types.
#[derive(Debug, Clone)]
pub struct Vector {
    /// Component type of the vector.
    pub component: Rc<SpirvType>,
    /// Component count of the vector.
    pub component_count: u32,
}

impl Vector {
    /// Create a new vector.
    pub fn new<T: 'static + SpirvType>(component: T, component_count: u32) -> Vector {
        Vector {
            component: Rc::new(component),
            component_count: component_count,
        }
    }
}

impl SpirvType for Vector {
    fn register_type(&self, shader: &mut Shader) -> Result<Word> {
        let component_type = self.component.register_type(shader)?;

        shader.cache_op(
            OpKey::Vector {
                component_type: component_type,
                component_count: self.component_count,
            },
            |s| Ok(s.builder.type_vector(component_type, self.component_count)),
        )
    }

    fn width(&self) -> u32 {
        self.component.width() * self.component_count
    }

    fn row_count(&self) -> Option<u32> {
        Some(self.component_count)
    }

    fn matches(&self, other: &SpirvType) -> bool {
        if let Some(other) = other.as_vector() {
            self.component.matches(other.component.as_ref()) &&
                self.component_count == other.component_count
        } else {
            false
        }
    }

    fn as_vector_dims(&self) -> Option<VectorDims> {
        Some(VectorDims::new(self.component_count))
    }

    fn as_vector(&self) -> Option<Vector> {
        Some(self.clone())
    }

    fn display(&self) -> String {
        format!("vec{}[{}]", self.component_count, self.component.display())
    }

    #[cfg(feature = "vulkan")]
    fn as_vulkano_format(&self) -> Option<::vulkano::format::Format> {
        use vulkano::format::Format::*;

        if self.component.as_float().is_some() {
            if self.component_count == 2 {
                return Some(R32G32Sfloat);
            }

            if self.component_count == 3 {
                return Some(R32G32B32Sfloat);
            }

            if self.component_count == 4 {
                return Some(R32G32B32A32Sfloat);
            }
        }

        None
    }
}

/// Reflects a matrix.
#[derive(Debug, Clone)]
pub struct Matrix {
    /// Column type of the matrix.
    pub column_type: Rc<SpirvType>,
    /// Column count of the matrix.
    pub column_count: u32,
}

impl Matrix {
    /// Create a new matrix.
    pub fn new<T: 'static + SpirvType>(column_type: T, column_count: u32) -> Matrix {
        Matrix {
            column_type: Rc::new(column_type),
            column_count: column_count,
        }
    }
}

impl SpirvType for Matrix {
    fn matrix_times_matrix(&self, rhs: &SpirvType) -> Option<Matrix> {
        if let (Some(lhs_dims), Some(rhs_dims)) = (self.as_matrix_dims(), rhs.as_matrix_dims()) {
            return lhs_dims.matrix_mul_type(rhs_dims);
        }

        None
    }

    fn matrix_times_vector(&self, rhs: &SpirvType) -> Option<Vector> {
        if let (Some(lhs_dims), Some(rhs_dims)) = (self.as_matrix_dims(), rhs.as_vector_dims()) {
            return lhs_dims.vector_mul_type(rhs_dims);
        }

        None
    }

    fn register_type(&self, shader: &mut Shader) -> Result<Word> {
        let column_type = self.column_type.register_type(shader)?;

        shader.cache_op(
            OpKey::Matrix {
                column_type: column_type,
                column_count: self.column_count,
            },
            |s| Ok(s.builder.type_matrix(column_type, self.column_count)),
        )
    }

    /// Decorates this matrix as part of the struct.
    /// This is required to determine the complete layout of the struct.
    fn register_struct_extra(&self, id: Word, index: u32, shader: &mut Shader) -> Result<()> {
        shader.builder.member_decorate(
            id,
            index,
            Decoration::ColMajor,
            &[],
        );

        shader.builder.member_decorate(
            id,
            index,
            Decoration::MatrixStride,
            &[Operand::LiteralInt32(self.column_type.width())],
        );

        Ok(())
    }

    fn width(&self) -> u32 {
        self.column_type.width() * self.column_count
    }

    fn matches(&self, other: &SpirvType) -> bool {
        if let Some(other) = other.as_matrix() {
            self.column_type.matches(other.column_type.as_ref()) &&
                self.column_count == other.column_count
        } else {
            false
        }
    }

    fn as_matrix_dims(&self) -> Option<MatrixDims> {
        return self.column_type.row_count().map(|row_count| {
            MatrixDims::new(self.column_count, row_count)
        });
    }

    fn as_matrix(&self) -> Option<Matrix> {
        Some(self.clone())
    }

    fn display(&self) -> String {
        format!("mat{}[{}]", self.column_count, self.column_type.display())
    }
}

/// Reflects a struct of non-uniform member types.
#[derive(Debug)]
pub struct Struct {
    /// Name of the struct. Must be unique.
    pub name: &'static str,
    /// Members of the struct.
    pub members: Vec<Rc<StructMember>>,
}

impl SpirvType for Struct {
    fn register_type(&self, shader: &mut Shader) -> Result<Word> {
        let mut field_types: Vec<Word> = Vec::new();

        for m in &self.members {
            field_types.push(m.ty.register_type(shader)?);
        }

        shader.cache_op(
            OpKey::Struct {
                name: String::from(self.name),
                field_types: field_types.clone(),
            },
            |s| {
                let id = s.builder.type_struct(&field_types);

                s.name(id, self.name);

                let mut offset = 0u32;

                for (index, ref m) in self.members.iter().enumerate() {
                    let index = index as u32;

                    s.member_name(id, index, m.name);

                    s.builder.member_decorate(
                        id,
                        index,
                        Decoration::Offset,
                        &[Operand::LiteralInt32(offset)],
                    );

                    offset += m.ty.width();
                    m.ty.register_struct_extra(id, index, s)?;
                }

                s.builder.decorate(id, Decoration::Block, &[]);
                Ok(id)
            },
        )
    }

    fn width(&self) -> u32 {
        self.members.iter().map(|m| m.ty.width()).sum()
    }

    fn matches(&self, other: &SpirvType) -> bool {
        if let Some(other) = other.as_struct() {
            if self.name != other.name {
                return false;
            }

            let mut a = self.members.iter();
            let mut b = other.members.iter();

            while let (Some(a), Some(b)) = (a.next(), b.next()) {
                if !a.matches(b) {
                    return false;
                }
            }

            return a.next().is_none() && b.next().is_none();
        } else {
            false
        }
    }

    fn display(&self) -> String {
        format!("struct {}", self.name)
    }
}
