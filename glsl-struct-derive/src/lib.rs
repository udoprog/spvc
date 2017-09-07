extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    let mut toks = quote::Tokens::new();

    toks.append(format!("impl {} {{", &ast.ident));

    let fields = {
        let mut out = Vec::new();

        if let syn::Body::Struct(syn::VariantData::Struct(ref fields)) = ast.body {
            for (index, ref field) in fields.iter().enumerate() {
                if let Some(ref ident) = field.ident {
                    out.push((index, ident.to_owned()));
                }
            }
        }

        out
    };

    for &(index, ref ident) in &fields {
        toks.append(quote!(const #ident: glsl_struct::GlslStructMember =
            glsl_struct::GlslStructMember {
                name: stringify!(#name),
                ty: glsl_struct::GlslType::Mat4,
                index: #index as u32,
            };
        ));
    }

    toks.append("fn glsl_struct() -> glsl_struct::GlslStruct {");

    toks.append("let mut members = Vec::new();");

    for &(_, ref ident) in &fields {
        toks.append(quote! { members.push(#name::#ident); });
    }

    toks.append(quote! {
        glsl_struct::GlslStruct {
            name: stringify!(#name),
            members: members,
        }
    });

    toks.append("}");
    toks.append("}");

    toks
}

#[proc_macro_derive(GlslStruct)]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_hello_world(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}
