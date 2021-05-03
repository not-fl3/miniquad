mod layout;

use {
    proc_macro::TokenStream,
    syn::{parse_macro_input, DeriveInput},
};

/// Adds `pub const VERTEX_ATTTRIBUTES: &'static [VertexAttribute]`
///
/// It requires the struct to be marked as `#[repr(C)]`.
#[proc_macro_derive(VertexLayout)]
pub fn vertex_layout(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    TokenStream::from(layout::impl_vertex_layout(ast))
}

