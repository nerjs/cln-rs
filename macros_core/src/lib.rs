extern crate proc_macro;
extern crate quote;
extern crate syn;

mod classnames_parser;
mod to_token_classnames;
mod tokens_parsers;

use proc_macro2::TokenStream;

use classnames_parser::IntoCnParser;
use quote::ToTokens;
pub use syn::Error;
use syn::{parse2, Result};
use to_token_classnames::IntoCnTypes;
use tokens_parsers::ChunkList;

pub fn cn_impl(input: proc_macro::TokenStream) -> Result<TokenStream> {
    let result = parse2::<ChunkList>(TokenStream::from(input))?
        .into_cn_parser()?
        .into_cn_types()
        .to_token_stream();
    Ok(result)
}
