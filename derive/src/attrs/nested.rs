/*
    Appellation: nested <module>
    Contrib: @FL03
*/
use super::DisplayAttr;
use syn::Ident;
use syn::parse::{Parse, ParseStream};

//[`Meta`] for key-value pairs

/// [`NestedAttr`] is an enumeration of various nested attributes the crate recognizes.
#[derive(Debug)]
pub enum NestedAttr {
    Display(DisplayAttr),
}

impl NestedAttr {
    /// attempts to parse the attribute from the given metadata
    pub fn parse_nested(meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<Self> {
        // #[contained(display(...))]
        if meta.path.is_ident("display") {
            let attr = DisplayAttr::parse_nested(meta)?;
            return Ok(Self::Display(attr));
        }

        Err(meta.error("unrecognized repr"))
    }
}

impl Parse for NestedAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        if ident == "display" {
            dbg!("found display attribute ");
            let content;
            syn::parenthesized!(content in input);
            // Parse an optional identifier
            let format = if content.is_empty() {
                None
            } else {
                Some(content.parse::<Ident>()?)
            };

            Ok(NestedAttr::Display(DisplayAttr { format }))
        } else {
            Err(syn::Error::new_spanned(ident, "unknown attribute"))
        }
    }
}
