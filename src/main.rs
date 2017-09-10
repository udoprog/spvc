extern crate rspirv;
extern crate spvc_shaders;

use self::rspirv::binary::Assemble;
use self::rspirv::binary::Disassemble;
use spvc_shaders::pbr;
use std::fs::File;
use std::io::Write;
use std::slice;

fn main() {
    let module = pbr::vertex_shader().unwrap().module();

    let code = module.assemble();
    assert!(code.len() > 20);

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
