use super::spirv::{StorageClass, Word};

/// Description of a SPIR-V type, used as a lookup key to avoid duplicate declarations.
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum TypeKey {
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
    GlobalVar {
        storage_class: StorageClass,
        variable_type: Word,
        set: Option<u32>,
        binding: Option<u32>,
        location: Option<u32>,
    },
    ConstantU32 { integer_type: Word, value: u32 },
    ConstantF32 { float_type: Word, value: u32 },
}
