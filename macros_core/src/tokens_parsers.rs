use proc_macro2::{Delimiter, Group, Span, TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    parse2,
    token::{And, AndAnd, Comma, Dot, Star},
    Error, Ident, LitBool, LitInt, LitStr, Result,
};

#[derive(Debug)]
pub struct ChunkGroup(Group);

impl Parse for ChunkGroup {
    fn parse(input: ParseStream) -> Result<Self> {
        let literal: TokenTree = input.parse()?;
        if let TokenTree::Group(group) = literal {
            if group.delimiter() != Delimiter::Parenthesis {
                return Err(Error::new_spanned(
                    group,
                    "Incorrect group delimiter. Allowed only parenthesis (..)",
                ));
            }

            return Ok(ChunkGroup(group.clone()));
        }

        Err(Error::new_spanned(
            literal,
            "Incorrect token group. allowed (..)",
        ))
    }
}

#[derive(Debug, Clone)]
pub struct ChunkIdent {
    pub sym: String,
    pub stream: TokenStream,
    pub is_fn: bool,
    pub is_composite: bool,
}

impl ChunkIdent {
    fn peek(input: ParseStream) -> bool {
        input.peek(Ident) || input.peek(And) || input.peek(AndAnd) || input.peek(Star)
    }
}

impl ToTokens for ChunkIdent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.stream.clone());
    }
}

impl Parse for ChunkIdent {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut sym = String::new();
        let mut stream = TokenStream::new();
        let mut is_fn = false;
        let mut is_composite = false;

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
                    is_composite = true;
                    let dot_literal: Dot = input.parse()?;
                    sym.push_str(".");
                    dot_literal.to_tokens(&mut stream);
                    expect_ident = true;
                } else {
                    let ChunkGroup(group) = input.parse()?;
                    stream.append(group);
                    is_fn = true;
                    expect_ident = false;
                }
            }
        }

        if expect_ident {
            return Err(Error::new(input.span(), "Expected ident"));
        }

        Ok(Self {
            sym,
            stream,
            is_fn,
            is_composite,
        })
    }
}

#[derive(Debug, Clone)]
pub enum ChunkTupleExp {
    Bool(bool),
    Ident(ChunkIdent),
}

impl Parse for ChunkTupleExp {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitBool) {
            let bool_token: LitBool = input.parse()?;
            return Ok(ChunkTupleExp::Bool(bool_token.value));
        } else if ChunkIdent::peek(input) {
            let ident: ChunkIdent = input.parse()?;
            return Ok(ChunkTupleExp::Ident(ident));
        } else {
            return Err(Error::new(
                input.span(),
                "Incorrect expression token. Allowed only bool or variable (bool, Option<bool>",
            ));
        }
    }
}

#[derive(Debug)]
pub struct ChunkTupleInner {
    pub exp: ChunkTupleExp,
    pub if_cond: String,
    pub else_cond: Option<String>,
}

impl Parse for ChunkTupleInner {
    fn parse(input: ParseStream) -> Result<Self> {
        let exp: ChunkTupleExp = input.parse()?;
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

#[derive(Debug, Clone)]
pub struct ChunkTuple {
    pub exp: ChunkTupleExp,
    pub if_cond: String,
    pub else_cond: Option<String>,
    pub span: Span,
}

impl ChunkTuple {
    fn set_span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }
}

impl From<ChunkTupleInner> for ChunkTuple {
    fn from(value: ChunkTupleInner) -> Self {
        Self {
            exp: value.exp,
            if_cond: value.if_cond,
            else_cond: value.else_cond,
            span: Span::call_site(),
        }
    }
}

impl Parse for ChunkTuple {
    fn parse(input: ParseStream) -> Result<Self> {
        let ChunkGroup(group) = input.parse()?;

        let chunk_tuple: ChunkTuple = parse2::<ChunkTupleInner>(group.stream())?.into();
        Ok(chunk_tuple.set_span(group.span()))
    }
}

#[derive(Debug, Clone)]
pub enum ChunkItem {
    Str { value: String, span: Span },
    Int { value: i32, span: Span },
    Ident(ChunkIdent),
    Tuple(ChunkTuple),
}

#[derive(Debug, Clone)]
pub struct ChunkList(pub Vec<ChunkItem>);

impl Parse for ChunkList {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut chunks: Vec<ChunkItem> = Vec::new();

        let mut expect_comma = false;
        while !input.is_empty() {
            if expect_comma {
                let _ = input.parse::<syn::token::Comma>()?;
            }

            if input.peek(LitStr) {
                let string: LitStr = input.parse()?;
                let value = string.value();
                chunks.push(ChunkItem::Str {
                    value,
                    span: string.span(),
                });
            } else if input.peek(LitInt) {
                let number: LitInt = input.parse()?;
                let value: i32 = number.base10_parse()?;
                chunks.push(ChunkItem::Int {
                    value,
                    span: number.span(),
                });
            } else if ChunkIdent::peek(input) {
                chunks.push(ChunkItem::Ident(input.parse::<ChunkIdent>()?))
            } else {
                chunks.push(ChunkItem::Tuple(input.parse::<ChunkTuple>()?))
            }

            expect_comma = true;
        }

        Ok(ChunkList(chunks))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quote::quote;
    use syn::parse2;

    fn all_is_ok<T: Parse>(list: Vec<TokenStream>) {
        for stream in list.into_iter() {
            let parsed = parse2::<T>(stream);
            assert!(parsed.is_ok());
        }
    }

    fn all_is_err<T: Parse>(list: Vec<TokenStream>) {
        for stream in list.into_iter() {
            let parsed = parse2::<T>(stream);
            assert!(parsed.is_err());
        }
    }

    trait IsBool {
        fn is_bool(&self) -> bool;
    }

    impl IsBool for ChunkTupleExp {
        fn is_bool(&self) -> bool {
            match self {
                ChunkTupleExp::Bool(_) => true,
                ChunkTupleExp::Ident(_) => false,
            }
        }
    }

    #[test]
    fn successfully_parse_group() {
        let list = vec![quote! { () }, quote! { ("some", "group") }];
        all_is_ok::<ChunkGroup>(list);
    }

    #[test]
    fn failed_parse_group() {
        let list = vec![
            quote! { "some", "group" },
            quote! { ["some", "group"] },
            quote! { {"some", "group"} },
        ];
        all_is_err::<ChunkGroup>(list);
    }

    #[test]
    fn successfully_parse_ident() {
        let list = vec![
            quote! { first },
            quote! { first.second },
            quote! { first.second() },
        ];
        all_is_ok::<ChunkIdent>(list);
    }

    #[test]
    fn failed_parse_ident() {
        let list = vec![
            quote! { "some" },
            quote! { first. },
            quote! { ("some", "group") },
        ];
        all_is_err::<ChunkIdent>(list);
    }

    #[test]
    fn check_ident_is_function() {
        let ident = parse2::<ChunkIdent>(quote! { variable() }).unwrap();

        assert_eq!(ident.sym, "variable");
        assert_eq!(ident.is_fn, true);
        assert_eq!(ident.is_composite, false);
    }

    #[test]
    fn check_ident_is_composite() {
        let ident = parse2::<ChunkIdent>(quote! { variable.some }).unwrap();

        assert_eq!(ident.sym, "variable.some");
        assert_eq!(ident.is_fn, false);
        assert_eq!(ident.is_composite, true);
    }

    #[test]
    fn successfully_parse_tuple() {
        let list = vec![
            quote! { (true, "first") },
            quote! { (first.second, "first") },
            quote! { (first.second(), "first", "second") },
            quote! { (false, "first", "second") },
        ];
        all_is_ok::<ChunkTuple>(list);
    }

    #[test]
    fn failed_parse_tuple() {
        let list = vec![
            quote! { "some" },
            quote! { first. },
            quote! { ("some", "group") },
            quote! { (true) },
            quote! { (some.var) },
        ];
        all_is_err::<ChunkTuple>(list);
    }

    #[test]
    fn successfully_parse_list() {
        let list = vec![
            quote! { "once", "string" },
            quote! { (true, "first", "second"), variable.var() },
            quote! { var.some(), "123", (false, "oo", "pp") },
        ];
        all_is_ok::<ChunkList>(list);
    }

    #[test]
    fn tuple_has_else_cond() {
        let tuple = parse2::<ChunkTuple>(quote! {(true, "first", "second")}).unwrap();
        assert_eq!(tuple.else_cond, Some("second".to_string()));
    }

    #[test]
    fn tuple_exp_is_bool() {
        let tuple = parse2::<ChunkTuple>(quote! {(true, "first", "second")}).unwrap();
        assert!(tuple.exp.is_bool());
    }

    #[test]
    fn failed_parse_list() {
        let list = vec![
            quote! { "some" "string" },
            quote! { "some"; "string" },
            quote! { first. },
            quote! { ("some", "group") },
            quote! { (true) },
            quote! { (some.var) },
        ];
        all_is_err::<ChunkList>(list);
    }
}
