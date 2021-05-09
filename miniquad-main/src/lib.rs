use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// we offer a convenience wrapper to be used as
/// ```
/// #[miniquad::main]
/// pub fn main() {}
/// ```
/// Basically we just wrap the ndk glue main where
/// we specify our custom glue code. Alternatively, you can do
/// this yourself with:
/// ```
/// #[cfg_attr(target_os = "android", ndk_glue::main(ndk_glue = "::miniquad::sapp_android"))]
/// pub fn main() {}
/// ```
#[proc_macro_attribute]
pub fn main(attr_input: TokenStream, item_input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item_input as ItemFn);

    let expanded = quote! {
        #[cfg_attr(target_os = "android", ::miniquad::sapp_android::ndk_glue::main(ndk_glue = "::miniquad::sapp_android"))]
        #input
    };

    TokenStream::from(expanded)
}
