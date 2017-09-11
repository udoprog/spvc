use super::global_var::GlobalVar;

#[derive(Debug, Clone, Copy)]
pub enum Interface<'a> {
    Input { var: &'a GlobalVar, location: u32 },
    Output { var: &'a GlobalVar, location: u32 },
    Uniform {
        var: &'a GlobalVar,
        set: u32,
        binding: u32,
    },
    BuiltIn,
}
