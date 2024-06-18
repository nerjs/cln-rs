use macros_core::{cn_impl, variant_impl, Error};
use proc_macro::TokenStream;

#[proc_macro]
pub fn cn(input: TokenStream) -> TokenStream {
    cn_impl(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Variant, attributes(class))]
pub fn derive_variant(input: TokenStream) -> TokenStream {
    variant_impl(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
