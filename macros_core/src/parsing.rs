use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Data, DeriveInput, Error, Generics, Ident, LitInt, LitStr, Result,
};

use crate::{
    fields::VariantFields,
    items::{CnIdent, CnTuple},
};

pub trait CnIntoBuilder {
    fn from_cn(items: Vec<CnItem>) -> Self;
}

impl CnParser {
    pub fn into_builder<T>(&self) -> T
    where
        T: CnIntoBuilder,
    {
        T::from_cn(self.0.clone())
    }
}

// Variant

pub struct VariantDeriveParser {
    pub name: Ident,
    pub fields: Vec<VariantFields>,
}

fn assert_with_generics(generics: &Generics) -> Result<()> {
    if generics.lt_token.is_some()
        || generics.gt_token.is_some()
        || !generics.params.is_empty()
        || generics.where_clause.is_some()
    {
        return Err(Error::new(
            generics.span(),
            "Support for enum with generics not yet implemented",
        ));
    }
    Ok(())
}

impl Parse for VariantDeriveParser {
    fn parse(input: ParseStream) -> Result<Self> {
        let input = input.parse::<DeriveInput>()?;
        assert_with_generics(&input.generics)?;
        match input.data {
            Data::Enum(data) => {
                let mut fields: Vec<VariantFields> = Vec::new();

                for variant in &data.variants {
                    fields.push(variant.clone().try_into()?);
                }

                return Ok(Self {
                    name: input.ident,
                    fields,
                });
            }
            _ => Err(Error::new(
                input.span(),
                "Only enums are supported for the variant",
            )),
        }
    }
}
