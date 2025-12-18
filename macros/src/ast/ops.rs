/*
    appellation: ops <module>
    authors: @FL03
*/
use crate::ast::WrapperOpsAst;

use syn::parse::{Parse, ParseStream};
use syn::token::Impl;
use syn::{Ident, Token, braced};

impl Parse for WrapperOpsAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // parse the `impl` keyword
        let _impl = input.parse::<Impl>()?;
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
        // parse the operations defined within braces
        let content;
        braced!(content in input);
        let mut ops = Vec::new();
        while !content.is_empty() {
            let op: Ident = content.parse()?;
            content.parse::<Token![.]>()?;
            let call: Ident = content.parse()?;
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
            ops.push((op, call));
        }
        Ok(Self {
            _impl,
            generics,
            target,
            field,
            where_clause,
            ops,
        })
    }
}
