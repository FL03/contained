/*
    Appellation: nested <module>
    Contrib: @FL03
*/
use super::DisplayAttr;
use syn::Ident;
use syn::parse::{Parse, ParseStream};

/// [`NestedAttr`] is an enumeration of various nested attributes the crate recognizes.
#[derive(Debug)]
pub enum NestedAttr {
    Inner(DisplayAttr),
}

impl NestedAttr {
    /// attempts to parse the attribute from the given metadata
    pub fn parse_nested(meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<Self> {
        // #[wrapper(inner(...))]
        if meta.path.is_ident("inner") {
            let attr = DisplayAttr::parse_nested(meta)?;
            return Ok(Self::Inner(attr));
        }

        Err(meta.error("unrecognized repr"))
    }
}

impl Parse for NestedAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        if ident == "inner" {
            let content;
            syn::parenthesized!(content in input);
            // Parse an optional identifier
            let format = if content.is_empty() {
                None
            } else {
                Some(content.parse::<Ident>()?)
            };

            Ok(NestedAttr::Inner(DisplayAttr { format }))
        } else {
            Err(syn::Error::new_spanned(ident, "unknown attribute"))
        }
    }
}
