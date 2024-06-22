use proc_macro2::{Punct, Spacing, Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::Ident;

pub fn ident_by_num(num: &u8, span: Option<Span>) -> Ident {
    let span = span.unwrap_or_else(|| Span::call_site());
    let name = format!("var_{}", num);

    Ident::new(&name, span)
}

pub fn append_separated_coma(target_tokens: &mut TokenStream, list_tokens: &Vec<TokenStream>) {
    target_tokens.append_separated(list_tokens, Punct::new(',', Spacing::Alone));
}

pub(crate) fn global_deps() -> TokenStream {
    quote! {ui_helpers_rs::__private::}
}
