use super::errors::*;
use super::function::Function;
use super::primitives::UnsignedInteger;
use super::rspirv;
use super::spirv::{ExecutionModel, Word};
use super::spirv_type::SpirvType;
use super::storage_class::StorageClass;
use super::type_key::TypeKey;
use super::variable::Variable;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub struct Shader {
    /// Internal builder
    pub(crate) builder: rspirv::mr::Builder,
    /// Cached types, to only initialize each type once.
    type_cache: HashMap<TypeKey, Word>,
}

impl fmt::Debug for Shader {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Shader {{ }}")
    }
}


impl Shader {
    pub fn new() -> Shader {
        use super::spirv::Capability;
        use super::spirv::AddressingModel;
        use super::spirv::MemoryModel;

        let mut builder = rspirv::mr::Builder::new();

        builder.capability(Capability::Shader);
        builder.ext_inst_import(String::from("GLSL.std.450"));
        builder.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);

        Shader {
            builder: builder,
            type_cache: HashMap::new(),
        }
    }

    pub(crate) fn cached_type<I>(&mut self, ty: TypeKey, inserter: I) -> Result<Word>
    where
        I: Fn(&mut Self) -> Result<Word>,
    {
        if let Some(id) = self.type_cache.get(&ty) {
            return Ok(*id);
        }

        let id = inserter(self)?;
        self.type_cache.insert(ty, id);
        Ok(id)
    }

    pub(crate) fn constant_u32(&mut self, value: u32) -> Result<Word> {
        let integer_type = UnsignedInteger.register_type(self)?;

        self.cached_type(
            TypeKey::ConstantU32 {
                integer_type: integer_type,
                value: value,
            },
            |s| Ok(s.builder.constant_u32(integer_type, value)),
        )
    }

    pub(crate) fn register_pointer_type(
        &mut self,
        storage_class: StorageClass,
        pointee_type: Word,
    ) -> Result<Word> {
        self.cached_type(
            TypeKey::Pointer {
                storage_class: storage_class,
                pointee_type: pointee_type,
            },
            |s| {
                Ok(s.builder.type_pointer(
                    None,
                    storage_class.into(),
                    pointee_type,
                ))
            },
        )
    }

    pub(crate) fn member_name(&mut self, id: Word, index: u32, name: &str) {
        self.builder.member_name(id, index, name.to_string());
    }

    pub(crate) fn name(&mut self, id: Word, name: &str) {
        self.builder.name(id, name.to_string());
    }

    pub fn vertex_entry_point(
        &mut self,
        function: Function,
        interface: Vec<Rc<Box<Variable>>>,
    ) -> Result<()> {
        let interface = {
            let mut out = Vec::new();

            for i in interface {
                out.push(i.register_variable(self)?.variable_id(self)?);
            }

            out
        };

        let name = function.name.clone();
        let id = function.register_function(self)?;

        self.builder.entry_point(
            ExecutionModel::Vertex,
            id,
            name,
            interface,
        );

        Ok(())
    }

    pub fn module(self) -> rspirv::mr::Module {
        self.builder.module()
    }
}
