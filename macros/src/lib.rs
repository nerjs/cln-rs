extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::parse_macro_input;
use syn::{
    parse::ParseStream, punctuated::Punctuated, Attribute, Expr, ExprArray, ExprReference, ExprTry,
    Ident, LitStr, MetaList, Token, Visibility,
};
use utils::zzz;

#[proc_macro]
pub fn normalize_list(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let string = utils::normalize_list_impl(input.value());

    quote! {
        #string
    }
    .into()
}


#[proc_macro]
pub fn normalize_list_from_util(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let string = input.value();

    quote! {
        zzz!(#string)
    }
    .into()
}
