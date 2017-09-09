extern crate spirv_headers as spirv;
extern crate rspirv;
extern crate spvc_shader;
#[macro_use]
extern crate glsl_struct_derive;

use self::rspirv::binary::Assemble;
use self::rspirv::binary::Disassemble;
use self::spirv::BuiltIn;
use self::spvc_shader::*;
use self::spvc_shader::errors::*;
use std::fs::File;
use std::io::Write;
use std::slice;

#[derive(GlslStruct, Debug)]
pub struct Model {
    #[glsl(ty = "mat4")]
    model: spvc_shader::GlslMat4,
    #[glsl(ty = "vec4")]
    base_color_factor: spvc_shader::GlslVec4,
    #[glsl(ty = "bool")]
    use_base_color_texture: spvc_shader::GlslBool,
}

#[derive(GlslStruct, Debug)]
pub struct Global {
    #[glsl(ty = "mat4")]
    camera: spvc_shader::GlslMat4,
    #[glsl(ty = "mat4")]
    view: spvc_shader::GlslMat4,
    #[glsl(ty = "mat4")]
    projection: spvc_shader::GlslMat4,
}

fn build_vertex_shader() -> Result<self::rspirv::mr::Module> {
    let mut shader = Shader::new();

    let model = GlobalVar::new("model", Model::glsl_struct(), StorageClass::Uniform).build();

    let global = GlobalVar::new("global", Global::glsl_struct(), StorageClass::Uniform).build();

    let position = GlobalVar::new("location", vec3(), StorageClass::Input)
        .with_location(0)
        .build();

    let normal = GlobalVar::new("normal", vec3(), StorageClass::Input)
        .with_location(1)
        .build();

    let tex_coord = GlobalVar::new("tex_coord", vec2(), StorageClass::Input)
        .with_location(2)
        .build();

    let gl_position = GlobalVar::new("gl_Position", vec3(), StorageClass::Output)
        .with_built_in(BuiltIn::Position)
        .build();

    {
        let mut main = FunctionBuilder::new("main");
        let camera = global.access(Global::camera())?;
        let view = global.access(Global::view())?;

        let cameraview = Mul::new(camera.clone(), view);
        let again = Mul::new(camera.clone(), cameraview.clone());

        let loaded_position = Load::new(position.clone());

        main.op(again);
        main.op(Store::new(gl_position, loaded_position));

        shader.vertex_entry_point(
            main.returns_void(),
            vec![
                position.clone(),
                normal.clone(),
                tex_coord.clone(),
            ],
        )?;
    }

    Ok(shader.module())
}

fn main() {
    let module = build_vertex_shader().unwrap();

    let code = module.assemble();
    assert!(code.len() > 20);
    assert_eq!(self::spirv::MAGIC_NUMBER, code[0]);

    let mut loader = self::rspirv::mr::Loader::new();
    self::rspirv::binary::parse_words(&code, &mut loader).unwrap();
    let module = loader.module();

    let mut out = File::create("out.spv").unwrap();

    unsafe {
        let code = slice::from_raw_parts(code.as_ptr() as *const u8, code.len() * 4);
        out.write_all(code).unwrap();
    }

    println!("{}", module.disassemble());
}
