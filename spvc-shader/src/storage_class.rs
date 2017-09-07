use super::spirv;

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
