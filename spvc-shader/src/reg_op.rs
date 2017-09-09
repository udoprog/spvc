use super::errors::*;
use super::shader::Shader;
use super::spirv::Word;
use std::fmt;

/// Reflects a registered single statement that results in an object.
pub trait RegOp: fmt::Debug {
    /// Reflects the object ID of the statement.
    /// Using the object ID tends to destroy it.
    fn op_id(&self, shader: &mut Shader) -> Result<Option<Word>>;
}
