/*
    Appellation: display_attrs <module>
    Contrib: @FL03
*/
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DisplayAttr {
    pub format: Option<Ident>,
}

impl DisplayAttr {
    /// attempts to parse the attribute from the given metadata
    pub fn parse_nested(meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<Self> {
        let content: syn::parse::ParseBuffer<'_>;
        syn::parenthesized!(content in meta.input);
        // try finding the optional name parameter
        let format = content.parse::<Ident>();
        // create a new instance of ParamsAttr
        let parsed = DisplayAttr {
            format: format.ok(),
        };
        // return the parsed instance
        Ok(parsed)
    }
}

#[allow(dead_code)]
pub enum DisplayMeta {
    Format(LitStr),
}

impl Parse for DisplayMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        if ident == "format" {
            dbg!("found format attribute key");
            input.parse::<syn::Token![=]>()?;
            let lit: LitStr = input.parse()?;
            return Ok(Self::Format(lit));
        }
        Err(syn::Error::new_spanned(ident, "unknown field"))
    }
}
