#[macro_use]
extern crate error_chain;
extern crate rspirv;
extern crate spirv_headers;

pub mod spirv_shader_base;
pub mod errors;
pub mod glsl_struct;

pub use self::errors::Error;

/// Helper macro to define GLSL structs in SPIR-V.
///
/// ## Example
///
/// ```rust
/// glsl_struct!{
///     GlobalUniform {
///         camera: mat4,
///         view: mat4,
///         projection: mat4,
///     }
/// }
///
/// fn main() {
///   let mut shader = SpirvShaderBase::new();
///   shader.glsl_struct(GlobalUniform::glsl_struct_info());
/// }
/// ```
#[macro_export]
macro_rules! glsl_struct {
    ($name:ident { $($member:ident: $type:ident,)* }) => {
        struct $name {
            $($member: glsl_struct!(@rust_type_of $type),)*
        }

        impl $crate::glsl_struct::GlslStructInfo for $name {
            fn glsl_struct() -> $crate::glsl_struct::GlslStruct {
                use $crate::glsl_struct::GlslStructMember;

                let mut members = Vec::new();

                $(
                    members.push(GlslStructMember {
                        name: String::from(stringify!($member)),
                        ty: glsl_struct!(@type_of $type),
                    });
                )*

                $crate::glsl_struct::GlslStruct {
                    name: String::from(stringify!($name)),
                    members: members,
                }
            }
        }
    };

    (@rust_type_of bool) => {
        u32
    };

    (@type_of bool) => {
        $crate::glsl_struct::GlslType::Bool
    };

    (@rust_type_of mat4) => {
        [[f32; 4]; 4]
    };

    (@type_of mat4) => {
        $crate::glsl_struct::GlslType::Mat4
    };

    (@rust_type_of vec2) => {
        [f32; 2]
    };

    (@type_of vec2) => {
        $crate::glsl_struct::GlslType::Vec2
    };

    (@rust_type_of vec3) => {
        [f32; 3]
    };

    (@type_of vec3) => {
        $crate::glsl_struct::GlslType::Vec3
    };

    (@rust_type_of vec4) => {
        [f32; 4]
    };

    (@type_of vec4) => {
        $crate::glsl_struct::GlslType::Vec4
    };
}
