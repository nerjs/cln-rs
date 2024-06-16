extern crate proc_macro;
extern crate quote;
extern crate syn;

use macros_core::{ChunkList, IntoCnParser, IntoCnTypes};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, Error, Result};

fn cn_impl(chunk_list: ChunkList) -> Result<proc_macro2::TokenStream> {
    let result = chunk_list
        .into_cn_parser()?
        .into_cn_types()
        .to_token_stream();
    Ok(result)
}

#[proc_macro]
pub fn cn(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ChunkList);

    cn_impl(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
