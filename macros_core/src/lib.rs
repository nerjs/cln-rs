extern crate proc_macro;
extern crate quote;
extern crate syn;

// mod fields;
// mod parsing;
mod utils;
mod classnames {
    pub mod parsers;
    pub mod units;
}

mod variants {
    pub mod parsers;
    pub mod units;
}

use classnames::parsers::CnParser;
use proc_macro2::TokenStream;

use quote::ToTokens;
pub use syn::Error;
use syn::{parse2, Result};
use variants::parsers::VariantDeriveParser;

pub fn cn_impl(input: proc_macro::TokenStream) -> Result<TokenStream> {
    let result = parse2::<CnParser>(TokenStream::from(input))?
        .to_cn_tokens()?
        .to_token_stream();

    Ok(result)
}

pub fn variant_impl(input: proc_macro::TokenStream) -> Result<TokenStream> {
    let result = parse2::<VariantDeriveParser>(TokenStream::from(input))?.to_token_stream();

    Ok(result)
}
