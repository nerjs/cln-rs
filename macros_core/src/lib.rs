extern crate proc_macro;
extern crate quote;
extern crate syn;

// mod fields;
// mod parsing;
mod classnames {
    pub mod parsers;
    pub mod units;
}

use classnames::parsers::CnParser;
use proc_macro2::TokenStream;

use quote::ToTokens;
pub use syn::Error;
use syn::{parse2, Result};

pub fn cn_impl(input: proc_macro::TokenStream) -> Result<TokenStream> {
    let result = parse2::<CnParser>(TokenStream::from(input))?
        .to_cn_tokens()?
        .to_token_stream();

    Ok(result)
}

pub fn variant_impl(input: proc_macro::TokenStream) -> Result<TokenStream> {
    // let input = parse2::<DeriveInput>(TokenStream::from(input))?;
    let input = parse2::<TokenStream>(TokenStream::from(input)).unwrap();
    println!("input: {:#?}", input);

    Ok(TokenStream::new())
}
