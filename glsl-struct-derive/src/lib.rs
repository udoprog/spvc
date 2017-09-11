extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use std::u8;

#[derive(Debug)]
struct Field {
    index: u32,
    ident: syn::Ident,
    type_builder: quote::Tokens,
}

#[derive(Debug)]
struct GlslAttribute {
    ty: Option<String>,
}

impl GlslAttribute {
    pub fn type_builder(&self) -> Option<quote::Tokens> {
        if let Some(ref ty) = self.ty {
            let out = match ty.as_str() {
                "bool" => quote!(spvc_shader::Bool),
                "float" => quote!(spvc_shader::Float),
                "mat3" => quote!(spvc_shader::mat3()),
                "mat4" => quote!(spvc_shader::mat4()),
                "vec2" => quote!(spvc_shader::vec2()),
                "vec3" => quote!(spvc_shader::vec3()),
                "vec4" => quote!(spvc_shader::vec4()),
                s => panic!(format!("unsupported type: {}", s)),
            };

            return Some(out);
        }

        None
    }
}

fn field_type_builder(field: &syn::Field) -> Option<quote::Tokens> {
    if let syn::Ty::Path(_, ref path) = field.ty {
        let mut s = path.segments.iter().map(|s| &s.ident);

        if let (Some(m), Some(type_name)) = (s.next(), s.next()) {
            if m == "st" {
                let out = match type_name.to_string().as_str() {
                    "Bool" => quote!(spvc_shader::Bool),
                    "Float" => quote!(spvc_shader::Float),
                    "Mat3" => quote!(spvc_shader::mat3()),
                    "Mat4" => quote!(spvc_shader::mat4()),
                    "Vec2" => quote!(spvc_shader::vec2()),
                    "Vec3" => quote!(spvc_shader::vec3()),
                    "Vec4" => quote!(spvc_shader::vec4()),
                    s => panic!(format!("unsupported type: {}", s)),
                };

                return Some(out);
            }
        }
    }

    None
}

fn impl_glsl_member(field: &Field) -> quote::Tokens {
    let index = field.index;
    let ident = &field.ident;
    let type_builder = &field.type_builder;

    let mut toks = quote::Tokens::new();

    toks.append(quote!(
        fn #ident() -> spvc_shader::StructMember  {
            spvc_shader::StructMember {
                name: stringify!(#ident),
                ty: ::std::rc::Rc::new(#type_builder),
                index: #index,
            }
        }
    ));

    toks
}


fn impl_glsl_struct_fn(name: &syn::Ident, fields: &[Field]) -> quote::Tokens {
    let mut toks = quote::Tokens::new();

    toks.append("fn type_info() -> spvc_shader::Struct {");

    toks.append("let mut members = Vec::new();");

    for field in fields {
        let ident = &field.ident;
        toks.append(
            quote! { members.push(::std::rc::Rc::new(#name::#ident())); },
        );
    }

    toks.append(quote! {
        spvc_shader::Struct {
            name: stringify!(#name),
            members: members,
        }
    });

    toks.append("}");

    toks
}

fn glsl_attribute(attribute: &[syn::Attribute]) -> GlslAttribute {
    use self::syn::NestedMetaItem;
    use self::syn::MetaItem;

    let mut out = GlslAttribute { ty: None };

    if let Some(ref attribute) = attribute.first() {
        if let MetaItem::List(ref ident, ref values) = attribute.value {
            if ident == "glsl" {
                for v in values {
                    if let NestedMetaItem::MetaItem(MetaItem::NameValue(ref ident, ref value)) =
                        *v
                    {
                        if ident == "ty" {
                            if let syn::Lit::Str(ref value, _) = *value {
                                out.ty = Some(value.to_owned());
                            }
                        }
                    }
                }
            }
        }
    }

    out
}

fn resolve_fields(ast: &syn::DeriveInput) -> Vec<Field> {
    let mut out: Vec<Field> = Vec::new();

    if let syn::Body::Struct(syn::VariantData::Struct(ref fields)) = ast.body {
        if fields.len() > u8::MAX as usize {
            panic!(format!(
                "Too many members. Has {} but max is 255.",
                fields.len()
            ));
        }

        for (index, ref field) in fields.iter().enumerate() {
            let ident = field.ident.as_ref().expect("expected field identifier");
            let glsl_attribute = glsl_attribute(&field.attrs);

            let type_builder = glsl_attribute
                .type_builder()
                .or_else(|| field_type_builder(field))
                .expect(&format!("cannot identify type for field: {}", ident));

            out.push(Field {
                index: index as u32,
                ident: ident.to_owned(),
                type_builder: type_builder,
            });
        }
    }

    out
}

fn impl_glsl_struct(ast: &syn::DeriveInput) -> quote::Tokens {
    let fields = resolve_fields(ast);

    let name = &ast.ident;

    let mut toks = quote::Tokens::new();

    toks.append(format!("impl {} {{", &ast.ident));

    for field in &fields {
        toks.append(impl_glsl_member(field));
    }

    toks.append(impl_glsl_struct_fn(name, &fields));
    toks.append("}");

    toks
}

#[proc_macro_derive(GlslStruct, attributes(glsl))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    impl_glsl_struct(&ast).parse().unwrap()
}
