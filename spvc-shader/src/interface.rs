use super::input_var::InputVar;
use super::output_var::OutputVar;
use super::uniform_var::UniformVar;

#[derive(Debug, Clone)]
pub enum Interface<'a> {
    Input(&'a InputVar),
    Output(&'a OutputVar),
    Uniform(&'a UniformVar),
    BuiltIn,
}
