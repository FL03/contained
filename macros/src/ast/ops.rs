/*
    appellation: ops <module>
    authors: @FL03
*/
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Token, braced};

/// The abstract syntax tree for the `binary_wrapper` macro input;
/// e.g. `A { Add.add, Sub.sub }` or `B.field { Add.add, Sub.sub }`
pub struct WrapperOpsAst {
    pub target: Ident,
    pub field: Option<Ident>,
    pub ops: Vec<(Ident, Ident)>,
}

impl Parse for WrapperOpsAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target: Ident = input.parse()?;
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
        Ok(Self { target, field, ops })
    }
}
