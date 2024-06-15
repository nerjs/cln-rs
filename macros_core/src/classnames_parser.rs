use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Error, Result};

use crate::{
    to_token_classnames::{CnTypes, IntoCnTypes},
    tokens_parsers::{ChunkIdent, ChunkItem, ChunkList, ChunkTupleExp},
};

#[derive(Debug, Clone)]
pub struct ChunkTuppleCn {
    pub exp: ChunkIdent,
    pub if_cond: String,
    pub else_cond: Option<String>,
}

impl ToTokens for ChunkTuppleCn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ChunkTuppleCn {
            exp,
            if_cond,
            else_cond,
        } = self;
        let token_stream = match else_cond {
            Some(else_cond) => quote! {(#exp, #if_cond, #else_cond)},
            None => quote! {(#exp, if_cond)},
        };
        tokens.append_all(token_stream);
    }
}

#[derive(Debug, Clone)]
pub enum CnItem {
    Str(String),
    Ident(ChunkIdent),
    Tuple(ChunkTuppleCn),
}

#[derive(Debug, Clone)]
pub struct CnListParser {
    pub items: Vec<CnItem>,
    pub vars: Vec<String>,
}

impl CnListParser {
    pub fn from_chunks(chunks: Vec<ChunkItem>) -> Result<Self> {
        let mut vars: Vec<String> = Vec::new();
        let mut items: Vec<CnItem> = Vec::new();

        for item in chunks {
            match item {
                ChunkItem::Str { value, span } => {
                    let parsed_string = value.trim();
                    if parsed_string.is_empty() {
                        return Err(Error::new(span, "String is empty"));
                    }

                    items.push(CnItem::Str(parsed_string.to_string()));
                }
                ChunkItem::Int { value, span: _ } => items.push(CnItem::Str(value.to_string())),
                ChunkItem::Ident(ident) => {
                    items.push(CnItem::Ident(ident.clone()));
                    vars.push(ident.sym);
                }
                ChunkItem::Tuple(tuple) => match &tuple.exp {
                    ChunkTupleExp::Bool(exp) => {
                        let string = match exp {
                            true => tuple.if_cond,
                            false => tuple.else_cond.unwrap_or(String::new()),
                        };
                        if string.is_empty() {
                            return Err(Error::new(
                                tuple.span,
                                "You cannot set blank lines for pre-defined conditions",
                            ));
                        }

                        items.push(CnItem::Str(string));
                    }
                    ChunkTupleExp::Ident(ident) => {
                        vars.push(ident.sym.clone());
                        items.push(CnItem::Tuple(ChunkTuppleCn {
                            exp: ident.clone(),
                            if_cond: tuple.if_cond,
                            else_cond: tuple.else_cond,
                        }));
                    }
                },
            }
        }

        Ok(Self { items, vars })
    }
}

pub trait IntoCnParser {
    fn into_cn_parser(&self) -> Result<CnListParser>;
}

impl IntoCnParser for ChunkList {
    fn into_cn_parser(&self) -> Result<CnListParser> {
        CnListParser::from_chunks(self.0.clone())
    }
}

impl IntoCnTypes for CnListParser {
    fn into_cn_types(&self) -> CnTypes {
        CnTypes::from_items(self.items.clone())
    }
}
