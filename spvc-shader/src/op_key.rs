use super::spirv::{BuiltIn, StorageClass, Word};

/// Description of a SPIR-V type, used as a lookup key to avoid duplicate declarations.
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum OpKey {
    Void,
    Bool,
    UnsignedInteger { width: u32 },
    Float { width: u32 },
    Vector {
        component_type: Word,
        component_count: u32,
    },
    Struct {
        name: String,
        field_types: Vec<Word>,
    },
    Matrix {
        column_type: Word,
        column_count: u32,
    },
    Pointer {
        storage_class: StorageClass,
        pointee_type: Word,
    },
    Function {
        return_type: Word,
        parameter_types: Vec<Word>,
    },
    InputVar { variable_type: Word, location: u32 },
    OutputVar { variable_type: Word, location: u32 },
    BuiltInVar {
        variable_type: Word,
        built_in: BuiltIn,
        storage_class: StorageClass,
    },
    UniformVar {
        variable_type: Word,
        set: u32,
        binding: u32,
    },
    ConstantU32 { integer_type: Word, value: u32 },
    ConstantF32 { float_type: Word, value: u32 },
}
