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

    let model = UniformVar::new("model", Model::type_info(), 0, 0);
    let global = UniformVar::new("global", Global::type_info(), 1, 0);

    let position = InputVar::new("location", vec3(), 0);
    let normal = InputVar::new("normal", vec3(), 1);
    let tex_coord = InputVar::new("tex_coord", vec2(), 2);

    let v_normal = OutputVar::new("v_normal", vec3(), 0);
    let v_tex_coord = OutputVar::new("v_tex_coord", vec2(), 1);

    let gl_position = BuiltInVar::new(
        "gl_Position",
        vec4(),
        StorageClass::Output,
        BuiltIn::Position,
    );

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
