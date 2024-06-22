use proc_macro2::{Punct, Spacing, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Data, DeriveInput, Error, Generics, Ident, Result,
};

use crate::utils::global_deps;

use super::units::VariantFields;


#[cfg_attr(feature = "debug", derive(Debug))]
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

impl ToTokens for VariantDeriveParser {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let mut variants = TokenStream::new();
        let mut fields: Vec<TokenStream> = Vec::new();

        for field in &self.fields {
            fields.push(field.to_token_stream());
        }

        variants.append_separated(fields, Punct::new(',', Spacing::Alone));

        let global_dep = global_deps();
        tokens.append_all(quote! {
            impl Into<#global_dep CnPart> for #name {
                fn into(self) -> #global_dep CnPart {
                    let variant = match self {
                        #variants
                    };

                    #global_dep CnPart::new(variant)
                }
            }
        })
    }
}
