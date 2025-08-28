/*
    Appellation: display_attrs <module>
    Contrib: @FL03
*/
use syn::parse::{Parse, ParseBuffer, ParseStream};
use syn::{parenthesized, Ident};
use syn::meta::ParseNestedMeta;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DisplayAttr {
    pub format: Option<Ident>,
}

impl DisplayAttr {
    /// attempts to parse the attribute from the given metadata
    pub fn parse_nested(meta: &ParseNestedMeta<'_>) -> syn::Result<Self> {
        let content;
        parenthesized!(content in meta.input);
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
