use proc_macro2::{Delimiter, Group, Span, TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    parse2,
    token::{And, AndAnd, Comma, Dot, Star},
    Error, Ident, LitBool, LitInt, LitStr, Result,
};

#[cfg_attr(feature = "debug", derive(Debug))]
pub struct CnGroup(Group);

impl Parse for CnGroup {
    fn parse(input: ParseStream) -> Result<Self> {
        let literal: TokenTree = input.parse()?;
        if let TokenTree::Group(group) = literal {
            if group.delimiter() != Delimiter::Parenthesis {
                return Err(Error::new_spanned(
                    group,
                    "Incorrect group delimiter. Allowed only parenthesis (..)",
                ));
            }

            return Ok(CnGroup(group.clone()));
        }

        Err(Error::new_spanned(
            literal,
            "Incorrect token group. allowed (..)",
        ))
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct CnIdent {
    pub sym: String,
    pub stream: TokenStream,
}

impl CnIdent {
    pub fn peek(input: ParseStream) -> bool {
        input.peek(Ident) || input.peek(And) || input.peek(AndAnd) || input.peek(Star)
    }
}

impl ToTokens for CnIdent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.stream.clone());
    }
}

impl Parse for CnIdent {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut sym = String::new();
        let mut stream = TokenStream::new();

        if input.peek(And) || input.peek(Star) {
            let prefix: TokenTree = input.parse()?;
            stream.append(prefix);
        }

        if input.peek(And) {
            let prefix: TokenTree = input.parse()?;
            stream.append(prefix);
        }

        let first_ident: Ident = input.parse()?;
        sym.push_str(&first_ident.to_string());
        stream.append(first_ident);

        let mut expect_ident = false;

        while !input.is_empty() {
            if expect_ident {
                let ident: Ident = input.parse()?;

                sym.push_str(&ident.to_string());
                stream.append(ident);
                expect_ident = false;
            } else {
                if input.peek(Comma) {
                    break;
                } else if input.peek(Dot) {
                    let dot_literal: Dot = input.parse()?;
                    sym.push_str(".");
                    dot_literal.to_tokens(&mut stream);
                    expect_ident = true;
                } else {
                    let CnGroup(group) = input.parse()?;
                    stream.append(group);
                    expect_ident = false;
                }
            }
        }

        if expect_ident {
            return Err(Error::new(input.span(), "Expected ident"));
        }

        Ok(Self { sym, stream })
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum CnTupleExp {
    Int(LitInt),
    Bool(LitBool),
    Ident(CnIdent),
}

impl Parse for CnTupleExp {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitBool) {
            return Ok(CnTupleExp::Bool(input.parse()?));
        } else if input.peek(LitInt) {
            return Ok(CnTupleExp::Int(input.parse()?));
        } else if CnIdent::peek(input) {
            return Ok(CnTupleExp::Ident(input.parse()?));
        } else {
            return Err(Error::new(
                input.span(),
                "Incorrect expression token. Allowed only bool or variable (bool, Option<bool>",
            ));
        }
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct CnTuple {
    pub exp: CnTupleExp,
    pub if_cond: String,
    pub else_cond: Option<String>,
    pub span: Span,
}

struct CnTupleInner {
    pub exp: CnTupleExp,
    pub if_cond: String,
    pub else_cond: Option<String>,
}

impl Parse for CnTupleInner {
    fn parse(input: ParseStream) -> Result<Self> {
        let exp: CnTupleExp = input.parse()?;
        let _ = input.parse::<Comma>()?;

        let if_cond = input.parse::<LitStr>()?.value().trim().to_string();

        let mut else_cond: Option<String> = None;
        if !input.is_empty() {
            let _ = input.parse::<Comma>()?;
            else_cond = Some(input.parse::<LitStr>()?.value().trim().to_string());
        }

        if !input.is_empty() {
            return Err(Error::new(
                input.span(),
                "Incorrect syntax. Only 3 elements allowed",
            ));
        }

        Ok(Self {
            exp,
            if_cond,
            else_cond,
        })
    }
}

impl Parse for CnTuple {
    fn parse(input: ParseStream) -> Result<Self> {
        let CnGroup(group) = input.parse()?;

        let CnTupleInner {
            exp,
            if_cond,
            else_cond,
        } = parse2::<CnTupleInner>(group.stream())?;
        Ok(Self {
            exp,
            if_cond,
            else_cond,
            span: group.span(),
        })
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum CnUnit {
    Str(LitStr),
    Int(LitInt),
    Ident(CnIdent),
    Tuple(CnTuple),
}
