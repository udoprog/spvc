use spvc_shader;
use spvc_shader::*;
use spvc_shader::errors::*;

#[derive(GlslStruct, Debug)]
#[repr(C)]
pub struct Model {
    model: st::Mat4,
    base_color_factor: st::Vec4,
    use_base_color_texture: st::Bool,
}

#[derive(GlslStruct, Debug)]
#[repr(C)]
pub struct Global {
    camera: st::Mat4,
    view: st::Mat4,
    projection: st::Mat4,
}

pub fn vertex_shader() -> Result<Shader> {
    let mut shader = Shader::new();

    let model = GlobalVar::new("model", Model::type_info(), StorageClass::Uniform).build();

    let global = GlobalVar::new("global", Global::type_info(), StorageClass::Uniform).build();

    let position = GlobalVar::new("location", vec3(), StorageClass::Input)
        .with_location(0)
        .build();

    let normal = GlobalVar::new("normal", vec3(), StorageClass::Input)
        .with_location(1)
        .build();

    let tex_coord = GlobalVar::new("tex_coord", vec2(), StorageClass::Input)
        .with_location(2)
        .build();

    let v_normal = GlobalVar::new("v_normal", vec3(), StorageClass::Output)
        .with_location(0)
        .build();

    let v_tex_coord = GlobalVar::new("v_tex_coord", vec2(), StorageClass::Output)
        .with_location(1)
        .build();

    let gl_position = GlobalVar::new("gl_Position", vec4(), StorageClass::Output)
        .with_built_in(BuiltIn::Position)
        .build();

    {
        let mut main = FunctionBuilder::new("main");
        let camera = global.access_member(Global::camera());
        let view = global.access_member(Global::view());

        let worldview = mul(load(view), load(camera.clone()));

        let pos = vec3_to_vec4(load(position.clone()), 1.0);
        let pos = mul(load(model.access_member(Model::model())), pos);
        let pos = mul(worldview, pos);
        let pos = mul(load(global.access_member(Global::projection())), pos);

        main.op(store(gl_position.clone(), pos));
        main.op(store(v_tex_coord.clone(), load(tex_coord.clone())));
        main.op(store(v_normal.clone(), load(normal.clone())));

        shader.entry_point(
            ShaderKind::Vertex,
            main.returns_void(),
            vec![
                position.clone(),
                normal.clone(),
                tex_coord.clone(),
                gl_position.clone(),
                v_normal.clone(),
                v_tex_coord.clone(),
            ],
        )?;
    }

    Ok(shader)
}
