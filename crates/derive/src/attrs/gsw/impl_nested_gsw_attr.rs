/*
    Appellation: impl_nested_attr <module>
    Created At: 2026.01.20:14:25:15
    Contrib: @FL03
*/
use crate::attrs::gsw::{GetAttr, NestedAttrGSW};
use syn::Ident;
use syn::parse::{Parse, ParseStream};

impl NestedAttrGSW {
    pub fn getter(format: Option<Ident>) -> Self {
        NestedAttrGSW::Get(GetAttr {
            format,
            mutable: false,
        })
    }
    /// attempts to parse the attribute from the given metadata
    pub fn parse_nested(meta: &syn::meta::ParseNestedMeta<'_>) -> syn::Result<Self> {
        // #[wrap(inner(...))]
        if meta.path.is_ident("inner") {
            let attr = GetAttr::parse_nested(meta)?;
            return Ok(Self::Get(attr));
        }

        Err(meta.error("unrecognized repr"))
    }
}

impl Parse for NestedAttrGSW {
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

            Ok(NestedAttrGSW::getter(format))
        } else {
            Err(syn::Error::new_spanned(ident, "unknown attribute"))
        }
    }
}
