use super::errors::*;
use super::rspirv;
use super::shader::Shader;
use super::spirv::{Decoration, Word};
use super::spirv_type::SpirvType;
use super::type_key::TypeKey;

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
}

#[derive(Debug, Clone, Copy)]
pub struct Vector<T> {
    pub component: T,
    pub component_count: u32,
}

impl<T> Vector<T> {
    pub fn new(component: T, component_count: u32) -> Vector<T> {
        Vector {
            component: component,
            component_count: component_count,
        }
    }
}

impl<T: SpirvType> SpirvType for Vector<T> {
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
}

#[derive(Debug, Clone, Copy)]
pub struct Matrix<T> {
    pub column_type: T,
    pub column_count: u32,
}

impl<T> Matrix<T> {
    pub fn new(column_type: T, column_count: u32) -> Matrix<T> {
        Matrix {
            column_type: column_type,
            column_count: column_count,
        }
    }
}

impl<T: SpirvType> SpirvType for Matrix<T> {
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

    fn width(&self) -> u32 {
        self.column_type.width() * self.column_count
    }

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
            vec![rspirv::mr::Operand::LiteralInt32(16)],
        );

        Ok(())
    }
}
