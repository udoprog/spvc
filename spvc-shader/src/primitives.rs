use super::errors::*;
use super::rspirv;
use super::shader::Shader;
use super::spirv::{Decoration, Word};
use super::spirv_type::SpirvType;
use super::type_key::TypeKey;
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

    fn width(&self) -> u32 {
        self.column_type.width() * self.column_count
    }

    fn matrix_dims(&self) -> Option<(u32, u32)> {
        return self.column_type.row_count().map(|row_count| {
            (self.column_count, row_count)
        });
    }
}
