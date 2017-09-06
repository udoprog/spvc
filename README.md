# spvc

This is an experimental high-level shader compiler for SPIR-V written in Rust.

The goal is to build useful shaders which can be configured dynamically depending on the model or
material used.

The shaders generated will be in immediate bytecode. No intermediate (GLSL, HLSL, ...)
representation will be needed which reduces the number of dependencies to a project (glslang).

## Utils

* [SPIRV-Tools][tools] - Assembler, disassembler, and
    validator.
* [rspirv][rspirv] - Low-level library handling bytecode assembly.

## How to work

Get `spirv-dis`, and `spirv-val` from [SPIRV-Tools][tools].
Also get glslangValidator from [glslang][glslang]. This will be used to generate reference spir-v
shaders.

GLSL shaders can be compiled with:

```bash
$> glslangValidator -V -l -o out.spv in{.vert,.frag}
```

Use `spirv-dis` to disassemble compiled shaders, and re-implement parts of them in this
project.

Rinse and repeat until stuff works.

## Shaders

Note: This project is currently in a very early stage, no shaders have been built yet.

#### PbrShader

Usage:

```rust
use spvc::{PbrModel, PbrShader, ShaderCache};

struct MyModel {
    // vertices, materials, uv maps, animation tracks, ...
}

impl PbrModel for MyModel {
    // implementation details
}

fn main() {
    let model = MyModel;
    let pbr = PbrShader::new();
    let shader_cache: ShaderCache<PipelineImpl> = ShaderCache::new(pbr);
    let shader = shader_cache.assemble_for(model);
    // use shader
}
```

[tools]: https://github.com/KhronosGroup/SPIRV-Tools
[glslang]: https://github.com/KhronosGroup/glslang
[rspirv]: https://github.com/google/rspirv
