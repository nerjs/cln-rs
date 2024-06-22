use proc_macro2::{Punct, Spacing, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};
use std::{collections::HashSet, num::TryFromIntError};
use syn::{
    parse2, spanned::Spanned, Error, Fields, FieldsNamed, FieldsUnnamed, Ident, Meta, Result,
    Variant,
};

use crate::{
    classnames::{
        parsers::{CheckVariantIdents, CheckVariantIndexes, CnParser, CnTokens},
        units::CnIdent,
    },
    utils::{append_separated_coma, ident_by_num},
};


#[cfg_attr(feature = "debug", derive(Debug))]
struct NamedParam {
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

fn get_valid_values_message_by_hashset(names: &HashSet<Ident>) -> String {
    let names_list = names
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>();
    format!("Alloved variant values: {:?}", names_list.join(", "))
}

impl CheckVariantIdents for NamedParam {
    fn check_ident(&mut self, value: CnIdent) -> Result<()> {
        if !self.names.contains(&value.ident) {
            return Err(Error::new(
                value.span(),
                get_valid_values_message_by_hashset(&self.names),
            ));
        }

        self.used.insert(value.ident);

        Ok(())
    }
}

impl ToTokens for NamedParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let params_list = self
            .names
            .clone()
            .into_iter()
            .map(|ident| {
                if self.used.contains(&ident) {
                    ident.to_token_stream()
                } else {
                    quote! { #ident : _ }
                }
            })
            .collect::<Vec<TokenStream>>();

        let mut inner_tokens = TokenStream::new();
        append_separated_coma(&mut inner_tokens, &params_list);
        tokens.append_all(quote! {{ #inner_tokens }});
    }
}


#[cfg_attr(feature = "debug", derive(Debug))]
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
                .map_err(|err: TryFromIntError| Error::new(value.span(), err.to_string()))?,
            used: HashSet::new(),
        })
    }
}

fn get_valid_values_message_by_count(max: u8) -> String {
    match max {
        0 => "Valid value only 0".to_string(),
        1 => "Valid values are 0 and 1".to_string(),
        _ => format!("Valid values from 0 to {}", max),
    }
}

impl CheckVariantIndexes for UnnamedParam {
    fn check_index(&mut self, value: u8, span: Span) -> Result<()> {
        let max_value = self.count - 1;
        if value > max_value {
            return Err(Error::new(
                span,
                get_valid_values_message_by_count(max_value),
            ));
        }
        self.used.insert(value);
        Ok(())
    }
}

impl ToTokens for UnnamedParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut params_list: Vec<TokenStream> = Vec::new();

        for num in 0..self.count {
            if self.used.contains(&num) {
                params_list.push(ident_by_num(&num, None).into_token_stream());
            } else {
                params_list.push(quote! {_})
            }
        }
        let mut inner_tokens = TokenStream::new();
        append_separated_coma(&mut inner_tokens, &params_list);
        tokens.append_all(quote! {( #inner_tokens )})
    }
}


#[cfg_attr(feature = "debug", derive(Debug))]
enum VariantParams {
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

impl CheckVariantIdents for VariantParams {
    fn check_ident(&mut self, value: CnIdent) -> Result<()> {
        match self {
            VariantParams::Named(named) => named.check_ident(value),
            _ => Err(Error::new(
                value.span(),
                "Only named parameters specified in the enum option are supported",
            )),
        }
    }
}

impl CheckVariantIndexes for VariantParams {
    fn check_index(&mut self, value: u8, span: Span) -> Result<()> {
        match self {
            VariantParams::Unnamed(unnamed) => unnamed.check_index(value, span),
            _ => Err(Error::new(
                span,
                "Only indices of non-named parameters specified in the enum variant are supported ",
            )),
        }
    }
}

impl ToTokens for VariantParams {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let stream = match self {
            VariantParams::None => TokenStream::new(),
            VariantParams::Unnamed(unnamed) => unnamed.to_token_stream(),
            VariantParams::Named(named) => named.to_token_stream(),
        };

        tokens.append_all(stream);
    }
}


#[cfg_attr(feature = "debug", derive(Debug))]
pub struct VariantFields {
    name: Ident,
    params: VariantParams,
    classes: CnTokens,
}

impl TryFrom<Variant> for VariantFields {
    type Error = Error;

    fn try_from(value: Variant) -> Result<Self> {
        let mut params: VariantParams = value.fields.clone().try_into()?;
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

        let classes = parse2::<CnParser>(tokens)?
            .check_idents(&mut params)?
            .to_cn_tokens()?;

        Ok(Self {
            name: value.ident.clone(),
            params,
            classes,
        })
    }
}

impl ToTokens for VariantFields {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let params = self.params.to_token_stream();
        let classes = self.classes.to_token_stream();
        tokens.append_all(quote! { Self:: #name #params => #classes});
    }
}
