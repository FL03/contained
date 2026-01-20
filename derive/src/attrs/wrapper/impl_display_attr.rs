/*
    Appellation: impl_display_attr <module>
    Created At: 2026.01.20:14:24:03
    Contrib: @FL03
*/
use crate::attrs::DisplayAttr;

use syn::meta::ParseNestedMeta;
use syn::{Ident, parenthesized};

impl DisplayAttr {
    /// attempts to parse the attribute from the given metadata
    pub fn parse_nested(meta: &ParseNestedMeta<'_>) -> syn::Result<Self> {
        let content;
        parenthesized! { content in meta.input };
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
