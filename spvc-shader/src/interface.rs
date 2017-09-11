use super::input_var::InputVar;
use super::output_var::OutputVar;
use super::uniform_var::UniformVar;

#[derive(Debug, Clone, Copy)]
pub enum Interface<'a> {
    Input { var: &'a InputVar, location: u32 },
    Output { var: &'a OutputVar, location: u32 },
    Uniform {
        var: &'a UniformVar,
        set: u32,
        binding: u32,
    },
    BuiltIn,
}
