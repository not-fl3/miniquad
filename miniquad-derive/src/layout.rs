use proc_macro2::TokenStream as TokenStream2;
use quote::*;
use syn::*;

pub fn impl_vertex_layout(ast: DeriveInput) -> TokenStream2 {
    let ty_name = &ast.ident;

    let input = match ast.data {
        Data::Struct(ref data) => data,
        _ => panic!("`#[derive(VertexLayout)]` is for structs"),
    };

    // force `#[repr(C)]`
    let repr: syn::Attribute = parse_quote!(#[repr(C)]);
    assert!(
        ast.attrs.iter().any(|a| *a == repr),
        "`#[repr(C)]` is required to derive `VertexLayout`"
    );

    // vertex attriubte/format
    let va = quote!(miniquad::graphics::VertexAttribute);
    let vf = quote!(miniquad::graphics::VertexFormat);

    // SUPPORTED FIELD TYPES ARE LIMITED TO THESE TYPES:
    let attr_decls = [
        ("f32", quote! { #vf::Float }),
        ("[f32; 1]", quote! { #vf::Float1 }),
        ("[f32; 2]", quote! { #vf::Float2 }),
        ("[f32; 3]", quote! { #vf::Float3 }),
        ("[f32; 4]", quote! { #vf::Float4 }),
        ("u8", quote! { #vf::Byte1 }),
        ("[u8; 1]", quote! { #vf::Byte1 }),
        ("[u8; 2]", quote! { #vf::Byte2 }),
        ("[u8; 3]", quote! { #vf::Byte3 }),
        ("[u8; 4]", quote! { #vf::Byte4 }),
        ("u16", quote! { #vf::Short1 }),
        ("[u16; 1]", quote! { #vf::Short1 }),
        ("[u16; 2]", quote! { #vf::Short2 }),
        ("[u16; 3]", quote! { #vf::Short3 }),
        ("[u16; 4]", quote! { #vf::Short4 }),
        ("[f32; 32]", quote! { #vf::Mat4 }),
    ];

    // maps type name tokens to vertex format
    let format_map = attr_decls
        .iter()
        .map(|(s, quote)| (syn::parse_str::<syn::Type>(s).unwrap(), quote));

    let fields = match input.fields {
        Fields::Named(ref fields) => fields,
        Fields::Unnamed(ref _fields) => todo!("impl `#[derive(VertexLayout)]` for tuple structs"),
        Fields::Unit => {
            unimplemented!("`#[derive(VertexLayout)]` for unit struct doesn't make sense!")
        }
    };

    let vtx_attrs = fields.named.iter().map(|field| {
        let format = format_map
            .clone()
            .find_map(|(ty, tokens)| if field.ty == ty { Some(tokens) } else { None })
            .unwrap_or_else(|| {
                // not found from the list
                panic!(
                    "Field `{}: {}` of type `{}` has unsupported type by `#[derive(VertexLayout)]`",
                    field.ident.as_ref().unwrap(),
                    field.ty.to_token_stream(),
                    ty_name,
                )
            });

        let field_ident = field.ident.as_ref().unwrap();
        let field_name = format!("{}", field_ident);

        quote! {
            #va::new(#field_name, #format)
        }
    });

    quote! {
        impl #ty_name {
            pub const VERTEX_ATTRIBUTES: &'static [#va] = &[#(#vtx_attrs,)*];
        }
    }
}
