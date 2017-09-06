extern crate rspirv;
extern crate spirv_headers as spirv;

use std::result;

use std::slice;
use rspirv::binary::Assemble;
use rspirv::binary::Disassemble;
use std::io::Write;
use std::fs::File;

#[derive(Debug)]
struct Error;

type Result<T> = result::Result<T, Error>;

impl From<rspirv::mr::Error> for Error {
    fn from(_: rspirv::mr::Error) -> Error {
        Error
    }
}

/// Builds gl_PerVertex structure.
fn gl_per_vertex(b: &mut rspirv::mr::Builder) -> Result<spirv::Word> {
    use self::rspirv::mr::Operand;
    use self::spirv::BuiltIn;
    use self::spirv::Decoration;

    let float_ = b.type_float(32);
    let uint_1 = b.type_int(1, 0);
    let arr_float_uint_1 = b.type_array(float_, uint_1);
    let v4float = b.type_array(float_, 4);

    let st = b.type_struct(vec![v4float, float_, arr_float_uint_1, arr_float_uint_1]);

    b.name(st, String::from("gl_PerVertex"));
    b.decorate(st, Decoration::Block, vec![]);

    b.member_name(st, 0, String::from("gl_Position"));
    b.member_name(st, 1, String::from("gl_PointSize"));
    b.member_name(st, 2, String::from("gl_ClipDistance"));
    b.member_name(st, 3, String::from("gl_CullDistance"));

    b.member_decorate(st, 0, spirv::Decoration::BuiltIn, vec![Operand::BuiltIn(BuiltIn::Position)]);
    b.member_decorate(st, 1, spirv::Decoration::BuiltIn, vec![Operand::BuiltIn(BuiltIn::PointSize)]);
    b.member_decorate(st, 2, spirv::Decoration::BuiltIn, vec![Operand::BuiltIn(BuiltIn::ClipDistance)]);
    b.member_decorate(st, 3, spirv::Decoration::BuiltIn, vec![Operand::BuiltIn(BuiltIn::CullDistance)]);

    Ok(st)
}

fn build_vertex_shader() -> Result<rspirv::mr::Module> {
    let mut b = rspirv::mr::Builder::new();

    b.capability(spirv::Capability::Shader);
    b.ext_inst_import(String::from("GLSL.std.450"));
    b.memory_model(spirv::AddressingModel::Logical, spirv::MemoryModel::GLSL450);

    let float_ = b.type_float(32);
    let v4float = b.type_array(float_, 4);
    let mat4v4float = b.type_matrix(v4float, 4);

    let gl_per_vertex = gl_per_vertex(&mut b)?;

    let ptr_output_gl_per_vertex = b.type_pointer(None, spirv::StorageClass::Output, gl_per_vertex);
    let _output = b.variable(ptr_output_gl_per_vertex, None, spirv::StorageClass::Output, None);

    let Global = b.type_struct(vec![mat4v4float, mat4v4float, mat4v4float]);
    b.name(Global, String::from("Global"));

    b.member_name(Global, 0, String::from("camera"));
    b.member_decorate(Global, 0, spirv::Decoration::ColMajor, vec![]);
    b.member_decorate(Global, 0, spirv::Decoration::Offset, vec![rspirv::mr::Operand::LiteralInt32(0)]);
    b.member_decorate(Global, 0, spirv::Decoration::MatrixStride, vec![rspirv::mr::Operand::LiteralInt32(16)]);

    b.member_name(Global, 1, String::from("view"));
    b.member_decorate(Global, 1, spirv::Decoration::ColMajor, vec![]);
    b.member_decorate(Global, 1, spirv::Decoration::Offset, vec![rspirv::mr::Operand::LiteralInt32(64)]);
    b.member_decorate(Global, 1, spirv::Decoration::MatrixStride, vec![rspirv::mr::Operand::LiteralInt32(16)]);

    b.member_name(Global, 2, String::from("projection"));
    b.member_decorate(Global, 2, spirv::Decoration::ColMajor, vec![]);
    b.member_decorate(Global, 2, spirv::Decoration::Offset, vec![rspirv::mr::Operand::LiteralInt32(128)]);
    b.member_decorate(Global, 2, spirv::Decoration::MatrixStride, vec![rspirv::mr::Operand::LiteralInt32(16)]);

    b.decorate(Global, spirv::Decoration::Block, vec![]);

    let v2float = b.type_vector(float_, 2);
    let ptr_output_v2float = b.type_pointer(None, spirv::StorageClass::Output, v2float);
    let v_tex_coord = b.variable(ptr_output_v2float, None, spirv::StorageClass::Output, None);
    b.name(v_tex_coord, String::from("v_tex_coord"));
    b.decorate(v_tex_coord, spirv::Decoration::Location, vec![rspirv::mr::Operand::LiteralInt32(0)]);

    let _ptr_Input_v2float = b.type_pointer(None, spirv::StorageClass::Input, v2float);
    let tex_coord = b.variable(_ptr_Input_v2float, None, spirv::StorageClass::Input, None);
    b.name(tex_coord, String::from("tex_coord"));
    b.decorate(tex_coord, spirv::Decoration::Location, vec![rspirv::mr::Operand::LiteralInt32(1)]);

    let _ptr_Uniform_Global = b.type_pointer(None, spirv::StorageClass::Uniform, Global);
    let global = b.variable(_ptr_Uniform_Global, None, spirv::StorageClass::Uniform, None);
    b.name(global, String::from("global"));
    b.decorate(global, spirv::Decoration::DescriptorSet, vec![rspirv::mr::Operand::LiteralInt32(0)]);
    b.decorate(global, spirv::Decoration::Binding, vec![rspirv::mr::Operand::LiteralInt32(0)]);

    let void = b.type_void();
    let voidf = b.type_function(void, vec![]);
    let main = b.begin_function(void, None, spirv::FUNCTION_CONTROL_NONE, voidf)?;

    b.name(main, String::from("main"));

    b.begin_basic_block(None)?;
    b.ret()?;
    b.end_function()?;

    b.entry_point(spirv::ExecutionModel::Vertex, main, String::from("main"), vec![_output, v_tex_coord, tex_coord]);

    Ok(b.module())
}

fn main() {
    let module = build_vertex_shader().unwrap();

    // Assembling
    let code = module.assemble();
    assert!(code.len() > 20);  // Module header contains 5 words
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
