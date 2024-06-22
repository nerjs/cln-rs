use crate::utils::global_deps;
use crate::utils::ident_by_num;

use super::units::{CnIdent, CnTupleExp, CnUnit};
use classlist::cleanup_cnl;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::{
    parse::{Parse, ParseStream},
    Error, LitInt, LitStr, Result,
};


#[cfg_attr(feature = "debug", derive(Debug))]
pub struct CnParser(pub Vec<CnUnit>);

impl Parse for CnParser {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut items: Vec<CnUnit> = Vec::new();

        let mut expect_comma = false;
        while !input.is_empty() {
            if expect_comma {
                let _ = input.parse::<syn::token::Comma>()?;
            }

            if input.peek(LitStr) {
                items.push(CnUnit::Str(input.parse()?));
            } else if input.peek(LitInt) {
                items.push(CnUnit::Int(input.parse()?));
            } else if CnIdent::peek(input) {
                items.push(CnUnit::Ident(input.parse::<CnIdent>()?))
            } else {
                items.push(CnUnit::Tuple(input.parse()?))
            }

            expect_comma = true;
        }

        Ok(Self(items))
    }
}

pub trait CheckVariantIndexes {
    fn check_index(&mut self, value: u8, span: Span) -> Result<()>;
}

pub trait CheckVariantIdents {
    fn check_ident(&mut self, value: CnIdent) -> Result<()>;
}

fn modify_int_ident<T: CheckVariantIndexes + CheckVariantIdents>(
    literal: LitInt,
    checker: &mut T,
) -> Result<CnIdent> {
    let value: u8 = literal.base10_parse()?;
    checker.check_index(value, literal.span())?;
    let ident = ident_by_num(&value, Some(literal.span()));

    Ok(CnIdent {
        sym: ident.to_string(),
        ident: ident.clone(),
        stream: ident.to_token_stream(),
    })
}

impl CnParser {
    pub fn check_idents<T: CheckVariantIndexes + CheckVariantIdents>(
        mut self,
        checker: &mut T,
    ) -> Result<Self> {
        let mut units: Vec<CnUnit> = Vec::new();

        for unit in self.0 {
            match unit {
                CnUnit::Str(_) => units.push(unit),
                CnUnit::Int(literal) => {
                    units.push(CnUnit::Ident(modify_int_ident(literal, checker)?));
                }
                CnUnit::Ident(ref ident) => {
                    checker.check_ident(ident.clone())?;
                    units.push(unit);
                }
                CnUnit::Tuple(mut tuple) => {
                    let exp = match tuple.exp {
                        CnTupleExp::Int(literal) => {
                            CnTupleExp::Ident(modify_int_ident(literal, checker)?)
                        }
                        CnTupleExp::Bool(_) => tuple.exp,
                        CnTupleExp::Ident(ref ident) => {
                            checker.check_ident(ident.clone())?;
                            tuple.exp
                        }
                    };

                    tuple.exp = exp;
                    units.push(CnUnit::Tuple(tuple));
                }
            }
        }

        self.0 = units;
        Ok(self)
    }

    pub fn to_cn_tokens(self) -> Result<CnTokens> {
        Ok(CnTokens::try_from(self)?)
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct CnIdentTupple {
    pub exp: TokenStream,
    pub if_cond: String,
    pub else_cond: Option<String>,
}

impl ToTokens for CnIdentTupple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let CnIdentTupple {
            exp,
            if_cond,
            else_cond,
        } = self;
        let token_stream = match else_cond {
            Some(else_cond) => quote! {(#exp, #if_cond, #else_cond)},
            None => quote! {(#exp, #if_cond)},
        };
        tokens.append_all(token_stream);
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum CnItem {
    Str(String),
    Ident(TokenStream),
    Tuple(CnIdentTupple),
}


#[cfg_attr(feature = "debug", derive(Debug))]
pub struct CnTokens(Vec<CnItem>);

fn merge_string(value: String, prefix_string: &mut String, items: &mut Vec<CnItem>) {
    let prepared_string = value.trim();
    if prepared_string.is_empty() {
        return;
    }
    if items.len() == 0 {
        prefix_string.push_str(&format!(" {}", prepared_string));
        return;
    }

    let last_item = items.last_mut();
    if let Some(item) = last_item {
        if let CnItem::Str(last_string) = item {
            last_string.push_str(&format!(" {}", prepared_string));
            return;
        }
    }

    items.push(CnItem::Str(prepared_string.to_string()));
}

impl TryFrom<CnParser> for CnTokens {
    type Error = Error;

    fn try_from(value: CnParser) -> Result<Self> {
        let mut items: Vec<CnItem> = Vec::new();
        let mut prefix_string = String::new();

        for unit in value.0 {
            match unit {
                CnUnit::Str(literal) => {
                    merge_string(literal.value(), &mut prefix_string, &mut items)
                }
                CnUnit::Int(literal) => {
                    merge_string(literal.to_string(), &mut prefix_string, &mut items)
                }
                CnUnit::Ident(ident) => items.push(CnItem::Ident(ident.stream)),
                CnUnit::Tuple(tuple) => match tuple.exp {
                    CnTupleExp::Ident(ident) => items.push(CnItem::Tuple(CnIdentTupple {
                        exp: ident.stream,
                        if_cond: tuple.if_cond,
                        else_cond: tuple.else_cond,
                    })),
                    CnTupleExp::Int(_) => {
                        return Err(Error::new(
                            tuple.span,
                            "Numeric identifier is not supported",
                        ))
                    }
                    CnTupleExp::Bool(bool_exp) => {
                        let value = match bool_exp.value {
                            true => Some(tuple.if_cond),
                            false => tuple.else_cond,
                        };

                        if let Some(tuple_string) = value {
                            merge_string(tuple_string, &mut prefix_string, &mut items)
                        }
                    }
                },
            }
        }

        let trimmed_prefix_string = prefix_string.trim();
        if !trimmed_prefix_string.is_empty() {
            items.push(CnItem::Str(cleanup_cnl(trimmed_prefix_string)));
            items.rotate_right(1);
        }

        Ok(Self(items))
    }
}

impl ToTokens for CnTokens {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.0.len() == 0 {
            tokens.append_all(quote! {String::new()});
            return;
        }

        if self.0.len() == 1 {
            if let Some(first) = self.0.first() {
                if let CnItem::Str(first_string) = first {
                    tokens.append_all(quote! { #first_string .to_string() });
                    return;
                }
            }
        }

        let global_dep = global_deps();
        tokens.append_all(quote! { #global_dep CnBuilder::new() });

        let stream_list = self
            .0
            .clone()
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
