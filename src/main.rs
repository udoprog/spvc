extern crate rspirv;
extern crate spirv_headers as spirv;
#[macro_use]
extern crate spvc;

use self::spvc::errors::*;
use self::spvc::spirv_shader_base::SpirvShaderBase;
use self::spvc::glsl_struct::GlslStructInfo;

use std::slice;
use rspirv::binary::Assemble;
use rspirv::binary::Disassemble;
use std::io::Write;
use std::fs::File;

glsl_struct!{Model {
    model: mat4,
    base_color_factor: vec4,
    use_base_color_texture: bool,
}}

glsl_struct!{Global {
    camera: mat4,
    view: mat4,
    projection: mat4,
}}

fn build_vertex_shader() -> Result<rspirv::mr::Module> {
    let mut shader = SpirvShaderBase::new();

    let float_ = shader.type_float(32);
    let v4float = shader.type_vector(float_, 4);
    let mat4v4float = shader.type_matrix(v4float, 4);

    let gl_per_vertex = shader.gl_per_vertex();

    let ptr_output_gl_per_vertex = shader.builder.type_pointer(
        None,
        spirv::StorageClass::Output,
        gl_per_vertex,
    );
    let _output = shader.builder.variable(
        ptr_output_gl_per_vertex,
        None,
        spirv::StorageClass::Output,
        None,
    );

    let model_type = shader.glsl_struct(Model::glsl_struct());
    let global_type = shader.glsl_struct(Global::glsl_struct());

    let v2float = shader.builder.type_vector(float_, 2);
    let ptr_output_v2float = shader.builder.type_pointer(
        None,
        spirv::StorageClass::Output,
        v2float,
    );
    let v_tex_coord = shader.builder.variable(
        ptr_output_v2float,
        None,
        spirv::StorageClass::Output,
        None,
    );
    shader.builder.name(
        v_tex_coord,
        String::from("v_tex_coord"),
    );
    shader.builder.decorate(
        v_tex_coord,
        spirv::Decoration::Location,
        vec![rspirv::mr::Operand::LiteralInt32(0)],
    );

    let ptr_input_v2float = shader.builder.type_pointer(
        None,
        spirv::StorageClass::Input,
        v2float,
    );

    let tex_coord = shader.builder.variable(
        ptr_input_v2float,
        None,
        spirv::StorageClass::Input,
        None,
    );

    shader.builder.name(tex_coord, String::from("tex_coord"));
    shader.builder.decorate(
        tex_coord,
        spirv::Decoration::Location,
        vec![rspirv::mr::Operand::LiteralInt32(1)],
    );

    let ptr_uniform_global = shader.builder.type_pointer(
        None,
        spirv::StorageClass::Uniform,
        global_type,
    );

    let global = shader.builder.variable(
        ptr_uniform_global,
        None,
        spirv::StorageClass::Uniform,
        None,
    );

    shader.builder.decorate(
        global,
        spirv::Decoration::DescriptorSet,
        vec![rspirv::mr::Operand::LiteralInt32(0)],
    );

    shader.builder.decorate(
        global,
        spirv::Decoration::Binding,
        vec![rspirv::mr::Operand::LiteralInt32(0)],
    );

    let void = shader.builder.type_void();
    let voidf = shader.builder.type_function(void, vec![]);
    let main = shader.builder.begin_function(
        void,
        None,
        spirv::FUNCTION_CONTROL_NONE,
        voidf,
    )?;

    shader.builder.name(main, String::from("main"));

    shader.builder.begin_basic_block(None)?;
    shader.builder.ret()?;
    shader.builder.end_function()?;

    shader.builder.entry_point(
        spirv::ExecutionModel::Vertex,
        main,
        String::from("main"),
        vec![_output, v_tex_coord, tex_coord],
    );

    Ok(shader.builder.module())
}

fn main() {
    let module = build_vertex_shader().unwrap();

    // Assembling
    let code = module.assemble();
    assert!(code.len() > 20); // Module header contains 5 words
    assert_eq!(spirv::MAGIC_NUMBER, code[0]);

    // Parsing
    let mut loader = rspirv::mr::Loader::new();
    rspirv::binary::parse_words(&code, &mut loader).unwrap();
    let module = loader.module();

    println!("writing out.spv");
    let mut out = File::create("out.spv").unwrap();

    unsafe {
        let code = slice::from_raw_parts(code.as_ptr() as *const u8, code.len() * 4);
        out.write_all(code).unwrap();
    }

    println!("{}", module.disassemble());
}
