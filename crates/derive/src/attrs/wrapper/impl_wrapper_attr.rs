/*
    Appellation: wrapper <module>
    Created At: 2026.01.20:14:20:08
    Contrib: @FL03
*/
use crate::attrs::{DisplayAttr, NestedAttr, WrapperAttr};
use syn::Attribute;

impl WrapperAttr {
    const BASEPATH: &'static str = "wrap";

    pub fn set_inner(&mut self, attr: DisplayAttr) {
        self.inner = Some(attr);
    }

    // tries to extract the root attribute from a list of attributes
    pub fn extract(attrs: &[Attribute]) -> syn::Result<Self> {
        let mut root = Self::default();
        for attr in attrs {
            if attr.path().is_ident(Self::BASEPATH) {
                attr.parse_nested_meta(|meta| {
                    if let Ok(nested) = NestedAttr::parse_nested(&meta) {
                        match nested {
                            NestedAttr::Inner(inner) => {
                                root.set_inner(inner);
                                return Ok(());
                            }
                        }
                    }
                    Err(meta.error("unrecognized attribute"))
                })?;
            }
        }
        Ok(root)
    }
}
