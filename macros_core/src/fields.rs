use proc_macro2::{Punct, Spacing, TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use std::collections::HashSet;
use syn::{
    spanned::Spanned, Error, Fields, FieldsNamed, FieldsUnnamed, Ident, Meta, Result, Variant,
};

#[derive(Debug)]
pub struct NamedParam {
    names: HashSet<Ident>,
    used: HashSet<Ident>,
}

impl From<FieldsNamed> for NamedParam {
    fn from(value: FieldsNamed) -> Self {
        let mut names: HashSet<Ident> = HashSet::new();

        for field in value.named {
            if let Some(ident) = field.ident {
                names.insert(ident);
            }
        }

        Self {
            names,
            used: HashSet::new(),
        }
    }
}

#[derive(Debug)]
struct UnnamedParam {
    count: u8,
    used: HashSet<u8>,
}

impl TryFrom<FieldsUnnamed> for UnnamedParam {
    type Error = Error;

    fn try_from(value: FieldsUnnamed) -> Result<Self> {
        Ok(Self {
            count: value
                .unnamed
                .len()
                .try_into()
                .map_err(|e| Error::new(value.span(), "message"))?,
            used: HashSet::new(),
        })
    }
}

#[derive(Debug)]
pub enum VariantParams {
    None,
    Unnamed(UnnamedParam),
    Named(NamedParam),
}

impl TryFrom<Fields> for VariantParams {
    type Error = Error;

    fn try_from(value: Fields) -> Result<Self> {
        let result = match value {
            Fields::Named(value) => Self::Named(value.into()),
            Fields::Unnamed(value) => Self::Unnamed(value.try_into()?),
            Fields::Unit => Self::None,
        };

        Ok(result)
    }
}

pub struct VariantFields {
    name: Ident,
    params: VariantParams,
}

impl TryFrom<Variant> for VariantFields {
    type Error = Error;

    fn try_from(value: Variant) -> Result<Self> {
        let params: VariantParams = value.fields.clone().try_into()?;
        let mut tokens = TokenStream::new();
        for attr in value.attrs {
            if let Meta::List(list) = attr.meta {
                if list.path.to_token_stream().to_string() != "class" {
                    continue;
                }

                if !list.tokens.is_empty() {
                    if !tokens.is_empty() {
                        tokens.append(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
                    }
                    tokens.append_all(list.tokens);
                }
            }
        }

        Ok(Self {
            name: value.ident.clone(),
            params,
        })
    }
}
