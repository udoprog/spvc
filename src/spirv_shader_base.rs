use rspirv;
use spirv_headers as spirv;
use self::spirv::Word;
use std::collections::HashMap;
use glsl_struct::GlslStruct;

pub struct SpirvShaderBase {
    /// Internal builder
    pub builder: rspirv::mr::Builder,
    /// gl_PerVertex cache
    gl_per_vertex: Option<Word>,
    /// Cached types, to only initialize each type once.
    type_cache: HashMap<SpirvType, Word>,
    /// Cached constant, to only initialize each type once.
    constant_cache: HashMap<SpirvConstant, Word>,
}

impl SpirvShaderBase {
    pub fn new() -> SpirvShaderBase {
        use self::spirv::Capability;
        use self::spirv::AddressingModel;
        use self::spirv::MemoryModel;

        let mut builder = rspirv::mr::Builder::new();

        builder.capability(Capability::Shader);
        builder.ext_inst_import(String::from("GLSL.std.450"));
        builder.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);

        SpirvShaderBase {
            builder: builder,
            gl_per_vertex: None,
            type_cache: HashMap::new(),
            constant_cache: HashMap::new(),
        }
    }

    fn cached_type<I>(&mut self, ty: SpirvType, inserter: I) -> Word
    where
        I: Fn(&mut rspirv::mr::Builder) -> Word,
    {
        let type_cache = &mut self.type_cache;
        let builder = &mut self.builder;

        *type_cache.entry(ty).or_insert_with(|| inserter(builder))
    }

    fn cached_constant<I>(&mut self, ty: SpirvConstant, inserter: I) -> Word
    where
        I: Fn(&mut rspirv::mr::Builder) -> Word,
    {
        let constant_cache = &mut self.constant_cache;
        let builder = &mut self.builder;

        *constant_cache.entry(ty).or_insert_with(
            || inserter(builder),
        )
    }

    pub fn type_bool(&mut self) -> Word {
        self.cached_type(SpirvType::Bool, |b| b.type_bool())
    }

    pub fn type_float(&mut self, width: u32) -> Word {
        self.cached_type(SpirvType::Float { width: width }, |b| b.type_float(width))
    }

    pub fn type_uint(&mut self, width: u32) -> Word {
        self.cached_type(SpirvType::UInt { width: width }, |b| b.type_int(width, 0))
    }

    pub fn constant_u32(&mut self, result_type: Word, value: u32) -> Word {
        self.cached_constant(
            SpirvConstant::U32 {
                result_type: result_type,
                value: value,
            },
            |b| b.constant_u32(result_type, value),
        )
    }

    pub fn type_array(&mut self, element_type: Word, length: Word) -> Word {
        self.cached_type(
            SpirvType::Array {
                element_type: element_type,
                length: length,
            },
            |b| b.type_array(element_type, length),
        )
    }

    pub fn type_vector(&mut self, component_type: Word, component_count: u32) -> Word {
        self.cached_type(
            SpirvType::Vector {
                component_type: component_type,
                component_count: component_count,
            },
            |b| b.type_vector(component_type, component_count),
        )
    }

    pub fn type_struct(&mut self, field_types: Vec<Word>) -> Word {
        self.cached_type(
            SpirvType::Struct { field_types: field_types.clone() },
            |b| b.type_struct(field_types.clone()),
        )
    }

    pub fn type_matrix(&mut self, column_type: Word, column_count: u32) -> Word {
        self.cached_type(
            SpirvType::Matrix {
                column_type: column_type,
                column_count: column_count,
            },
            |b| b.type_matrix(column_type, column_count),
        )
    }

    pub fn type_pointer<W: Into<Word>>(
        &mut self,
        storage_class: StorageClass,
        pointee_type: W,
    ) -> Word {
        let pointee_type = pointee_type.into();

        self.cached_type(
            SpirvType::Pointer {
                storage_class: storage_class,
                pointee_type: pointee_type,
            },
            |b| b.type_pointer(None, storage_class.into(), pointee_type),
        )
    }

    /// Build a struct that has been defined using the glsl_struct macro.
    pub fn glsl_struct(&mut self, type_info: GlslStruct) -> Word {
        let mut field_types = Vec::new();

        for m in &type_info.members {
            use super::glsl_struct::GlslType::*;

            match m.ty {
                Bool => {
                    field_types.push(self.type_bool());
                }
                Vec2 => {
                    let float_ = self.type_float(32);
                    let vec2 = self.type_vector(float_, 2);
                    field_types.push(vec2);
                }
                Vec3 => {
                    let float_ = self.type_float(32);
                    let vec3 = self.type_vector(float_, 3);
                    field_types.push(vec3);
                }
                Vec4 => {
                    let float_ = self.type_float(32);
                    let vec4 = self.type_vector(float_, 4);
                    field_types.push(vec4);
                }
                Mat4 => {
                    let float_ = self.type_float(32);
                    let v4float = self.type_vector(float_, 4);
                    let mat4 = self.type_matrix(v4float, 4);
                    field_types.push(mat4);
                }
            }
        }

        let st = self.type_struct(field_types);

        self.builder.name(st, type_info.name.to_string());

        let mut offset = 0u32;

        for (i, ref m) in type_info.members.iter().enumerate() {
            use super::glsl_struct::GlslType::*;

            let i = i as u32;

            self.builder.member_name(st, i as u32, m.name.to_string());

            self.builder.member_decorate(
                st,
                i,
                spirv::Decoration::Offset,
                vec![rspirv::mr::Operand::LiteralInt32(offset)],
            );

            match m.ty {
                Bool => {
                    offset += 4;
                }
                Vec2 => {
                    offset += 4 * 2;
                }
                Vec3 => {
                    offset += 4 * 3;
                }
                Vec4 => {
                    offset += 4 * 4;
                }
                Mat4 => {
                    self.builder.member_decorate(
                        st,
                        i,
                        spirv::Decoration::ColMajor,
                        vec![],
                    );

                    self.builder.member_decorate(
                        st,
                        i,
                        spirv::Decoration::MatrixStride,
                        vec![rspirv::mr::Operand::LiteralInt32(16)],
                    );

                    offset += 4 * 4 * 4;
                }
            }
        }

        self.builder.decorate(st, spirv::Decoration::Block, vec![]);
        st
    }

    /// Build the GLSL gl_PerVertex structure.
    pub fn gl_per_vertex(&mut self) -> Word {
        use self::rspirv::mr::Operand;
        use self::spirv::BuiltIn;
        use self::spirv::Decoration;

        if let Some(w) = self.gl_per_vertex {
            return w;
        }

        let float_ = self.type_float(32);
        let uint32 = self.type_uint(32);
        let uint_1 = self.constant_u32(uint32, 1);
        let arr_float_uint_1 = self.type_array(float_, uint_1);
        let v4float = self.type_vector(float_, 4);

        let st = self.type_struct(vec![v4float, float_, arr_float_uint_1, arr_float_uint_1]);

        self.builder.name(st, String::from("gl_PerVertex"));
        self.builder.decorate(st, Decoration::Block, vec![]);

        self.builder.member_name(st, 0, String::from("gl_Position"));
        self.builder.member_name(
            st,
            1,
            String::from("gl_PointSize"),
        );
        self.builder.member_name(
            st,
            2,
            String::from("gl_ClipDistance"),
        );
        self.builder.member_name(
            st,
            3,
            String::from("gl_CullDistance"),
        );

        self.builder.member_decorate(
            st,
            0,
            spirv::Decoration::BuiltIn,
            vec![Operand::BuiltIn(BuiltIn::Position)],
        );
        self.builder.member_decorate(
            st,
            1,
            spirv::Decoration::BuiltIn,
            vec![Operand::BuiltIn(BuiltIn::PointSize)],
        );
        self.builder.member_decorate(
            st,
            2,
            spirv::Decoration::BuiltIn,
            vec![Operand::BuiltIn(BuiltIn::ClipDistance)],
        );
        self.builder.member_decorate(
            st,
            3,
            spirv::Decoration::BuiltIn,
            vec![Operand::BuiltIn(BuiltIn::CullDistance)],
        );

        self.gl_per_vertex = Some(st);
        st
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum StorageClass {
    UniformConstant,
    Input,
    Uniform,
    Output,
    Workgroup,
    CrossWorkgroup,
    Private,
    Function,
    Generic,
    PushConstant,
    AtomicCounter,
    Image,
}

impl From<StorageClass> for spirv::StorageClass {
    fn from(value: StorageClass) -> spirv::StorageClass {
        use self::StorageClass::*;

        match value {
            UniformConstant => spirv::StorageClass::UniformConstant,
            Input => spirv::StorageClass::Input,
            Uniform => spirv::StorageClass::Uniform,
            Output => spirv::StorageClass::Output,
            Workgroup => spirv::StorageClass::Workgroup,
            CrossWorkgroup => spirv::StorageClass::CrossWorkgroup,
            Private => spirv::StorageClass::Private,
            Function => spirv::StorageClass::Function,
            Generic => spirv::StorageClass::Generic,
            PushConstant => spirv::StorageClass::PushConstant,
            AtomicCounter => spirv::StorageClass::AtomicCounter,
            Image => spirv::StorageClass::Image,
        }
    }
}

impl From<spirv::StorageClass> for StorageClass {
    fn from(value: spirv::StorageClass) -> StorageClass {
        use self::spirv::StorageClass::*;

        match value {
            UniformConstant => StorageClass::UniformConstant,
            Input => StorageClass::Input,
            Uniform => StorageClass::Uniform,
            Output => StorageClass::Output,
            Workgroup => StorageClass::Workgroup,
            CrossWorkgroup => StorageClass::CrossWorkgroup,
            Private => StorageClass::Private,
            Function => StorageClass::Function,
            Generic => StorageClass::Generic,
            PushConstant => StorageClass::PushConstant,
            AtomicCounter => StorageClass::AtomicCounter,
            Image => StorageClass::Image,
        }
    }
}

/// Description of a SPIR-V type, used for cache keys.
#[derive(PartialEq, Eq, Hash, Debug)]
enum SpirvType {
    Bool,
    Float { width: u32 },
    UInt { width: u32 },
    Vector {
        component_type: Word,
        component_count: u32,
    },
    Array { element_type: Word, length: Word },
    Struct { field_types: Vec<Word> },
    Matrix {
        column_type: Word,
        column_count: u32,
    },
    Pointer {
        storage_class: StorageClass,
        pointee_type: Word,
    },
}

/// Description of a SPIR-V type, used for cache keys.
#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
enum SpirvConstant {
    U32 { result_type: Word, value: u32 },
}
