/*
    Appellation: impl_getter_attr <module>
    Created At: 2026.01.20:14:31:21
    Contrib: @FL03
*/
use crate::attrs::GetAttr;

use syn::meta::ParseNestedMeta;
use syn::{Ident, parenthesized};

impl GetAttr {
    /// attempts to parse the attribute from the given metadata
    pub fn parse_nested(meta: &ParseNestedMeta<'_>) -> syn::Result<Self> {
        let content;
        parenthesized! { content in meta.input };
        // try finding the optional name parameter
        let format = content.parse::<Ident>();
        // create a new instance of ParamsAttr
        let parsed = GetAttr {
            format: format.ok(),
            mutable: false,
        };
        // return the parsed instance
        Ok(parsed)
    }
}
