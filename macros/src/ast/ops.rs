/*
    appellation: ops <module>
    authors: @FL03
*/
use syn::parse::{Parse, ParseStream};
use syn::token::Impl;
use syn::{Ident, Token, braced};

/// The abstract syntax tree for the `binary_wrapper` macro input;
/// e.g. `impl A { Add.add, Sub.sub }` or `impl B.field { Add.add, Sub.sub }`
pub struct WrapperOpsAst {
    pub _impl: Impl,
    pub target: Ident,
    pub field: Option<Ident>,
    pub ops: Vec<(Ident, Ident)>,
}

impl Parse for WrapperOpsAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _impl = input.parse::<Impl>()?;
        let target = input.parse::<Ident>()?;
        // resolve the optional named field
        let field = if input.peek(Token![.]) {
            input.parse::<Token![.]>()?;
            Some(input.parse()?)
        } else {
            None
        };
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
            target,
            field,
            ops,
        })
    }
}
