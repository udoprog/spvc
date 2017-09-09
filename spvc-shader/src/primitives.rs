use super::errors::*;
use super::matrix_dims::MatrixDims;
use super::rspirv::mr::Operand;
use super::shader::Shader;
use super::spirv::{Decoration, Word};
use super::spirv_type::SpirvType;
use super::type_key::TypeKey;
use super::vector_dims::VectorDims;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub struct Float;

impl SpirvType for Float {
    fn register_type(&self, shader: &mut Shader) -> Result<Word> {
        shader.cached_type(
            TypeKey::Float { width: 32 },
            |s| Ok(s.builder.type_float(32)),
        )
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
        shader.cached_type(TypeKey::UnsignedInteger { width: 32 }, |s| {
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

#[derive(Debug, Clone, Copy)]
pub struct Bool;

impl SpirvType for Bool {
    fn register_type(&self, shader: &mut Shader) -> Result<Word> {
        shader.cached_type(TypeKey::Bool, |s| Ok(s.builder.type_bool()))
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

#[derive(Debug, Clone)]
pub struct Vector {
    pub component: Rc<SpirvType>,
    pub component_count: u32,
}

impl Vector {
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

        shader.cached_type(
            TypeKey::Vector {
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
}

#[derive(Debug, Clone)]
pub struct Matrix {
    pub column_type: Rc<SpirvType>,
    pub column_count: u32,
}

impl Matrix {
    pub fn new<T: 'static + SpirvType>(column_type: T, column_count: u32) -> Matrix {
        Matrix {
            column_type: Rc::new(column_type),
            column_count: column_count,
        }
    }
}

impl SpirvType for Matrix {
    fn matrix_times_matrix(&self, rhs: &SpirvType) -> Result<Option<Matrix>> {
        if let (Some(lhs_dims), Some(rhs_dims)) = (self.as_matrix_dims(), rhs.as_matrix_dims()) {
            return lhs_dims.matrix_mul_type(rhs_dims).map(Some);
        }

        Ok(None)
    }

    fn matrix_times_vector(&self, rhs: &SpirvType) -> Result<Option<Vector>> {
        if let (Some(lhs_dims), Some(rhs_dims)) = (self.as_matrix_dims(), rhs.as_vector_dims()) {
            return lhs_dims.vector_mul_type(rhs_dims).map(Some);
        }

        Ok(None)
    }

    fn register_type(&self, shader: &mut Shader) -> Result<Word> {
        let column_type = self.column_type.register_type(shader)?;

        shader.cached_type(
            TypeKey::Matrix {
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
            vec![],
        );

        shader.builder.member_decorate(
            id,
            index,
            Decoration::MatrixStride,
            vec![Operand::LiteralInt32(self.column_type.width())],
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
