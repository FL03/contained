/*
    appellation: ops <module>
    authors: @FL03
*/
use crate::ast::{MethodCallAst, WrapperImpls};

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Impl;
use syn::{Ident, Token, braced};

fn _parse_ops(input: ParseStream) -> syn::Result<Punctuated<MethodCallAst, Token![,]>> {
    // parse the operations defined within braces
    let content;
    let _ = braced! { content in input };
    Punctuated::parse_terminated(&content)
}

impl Parse for MethodCallAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let period = input.parse::<Token![.]>()?;
        let call = input.parse::<Ident>()?;
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }
        Ok(Self {
            name,
            dot: period,
            call,
        })
    }
}

impl Parse for WrapperImpls {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // parse the `impl` keyword
        let impl_token = input.parse::<Impl>()?;
        // detect any generic parameters
        let generics = if input.peek(Token![<]) {
            input.parse().ok()
        } else {
            None
        };
        let target = input.parse::<Ident>()?;
        // resolve the optional named field
        let field = if input.peek(Token![.]) {
            input.parse::<Token![.]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        // parse the optional where clause
        let where_clause = if input.peek(Token![where]) {
            Some(input.parse()?)
        } else {
            None
        };
        // parse the operations block
        let content;
        let _ = braced! { content in input };
        let mut ops = Punctuated::new();
        while !content.is_empty() {
            ops.push(content.parse::<MethodCallAst>()?);
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            impl_token,
            generics,
            target,
            field,
            where_clause,
            ops,
        })
    }
}
