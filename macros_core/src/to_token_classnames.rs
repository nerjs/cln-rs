use crate::{classnames_parser::CnItem, cleanup_cnl::cleanup_cnl};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug)]
pub enum CnTypes {
    None,
    Once(String),
    Full(Vec<CnItem>),
}

impl CnTypes {
    pub fn from_items(items_list: Vec<CnItem>) -> Self {
        let mut prefix_string = String::new();
        let mut items: Vec<CnItem> = Vec::new();

        for item in items_list {
            match item {
                CnItem::Str(ref string) => {
                    if items.len() > 0 {
                        items.push(item);
                    } else {
                        prefix_string.push_str(&format!(" {}", string));
                    }
                }
                _ => items.push(item),
            }
        }

        let trimmed_prefix_string = prefix_string.trim();
        if items.len() == 0 {
            if trimmed_prefix_string.is_empty() {
                return Self::None;
            } else {
                return Self::Once(cleanup_cnl(trimmed_prefix_string));
            }
        }

        if !trimmed_prefix_string.is_empty() {
            items.push(CnItem::Str(cleanup_cnl(trimmed_prefix_string)));
            items.rotate_right(1);
        }

        Self::Full(items)
    }
}

pub trait IntoCnTypes {
    fn into_cn_types(&self) -> CnTypes;
}

impl ToTokens for CnTypes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            CnTypes::None => tokens.append_all(quote! {""}),
            CnTypes::Once(string) => tokens.append_all(quote! {#string}),
            CnTypes::Full(items) => {
                tokens.append_all(quote! { ui_helpers_rs::__private::CnBuilder::new() });

                let stream_list = items
                    .into_iter()
                    .map(|item| match item {
                        CnItem::Str(string) => quote! {#string},
                        CnItem::Ident(ident) => quote! {#ident},
                        CnItem::Tuple(tuple) => quote! {#tuple},
                    })
                    .map(|ts| quote! {.add(#ts)})
                    .collect::<Vec<TokenStream>>();

                tokens.append_all(stream_list);

                tokens.append_all(quote! {.to_classlist()})
            }
        }
    }
}
