/*
    appellation: wrapper <module>
    authors: @FL03
*/
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Token};

pub struct WrapperAst {
    pub target: Ident,
    pub field: Option<NamedFieldAst>,
}

pub struct NamedFieldAst {
    pub period: Token![.],
    pub field: Ident,
}

/*
 ************* Implementations *************
*/
impl Parse for NamedFieldAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let period = input.parse::<Token![.]>()?;
        let field = input.parse()?;
        Ok(Self { period, field })
    }
}

impl Parse for WrapperAst {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target: Ident = input.parse()?;
        // resolve the optional named field
        let field = if input.peek(Token![.]) {
            Some(input.parse()?)
        } else {
            None
        };
        Ok(Self { target, field })
    }
}
