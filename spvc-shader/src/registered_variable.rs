use super::errors::*;
use super::spirv::Word;
use std::fmt;

pub trait RegisteredVariable: fmt::Debug {
    fn variable_id(&self) -> Result<Word>;
}

impl RegisteredVariable for Word {
    fn variable_id(&self) -> Result<Word> {
        Ok(*self)
    }
}
