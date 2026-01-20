/*
    Appellation: impl_gsw_attr <module>
    Created At: 2026.01.20:14:35:19
    Contrib: @FL03
*/
use crate::attrs::gsw::GetSetWithAttr;
use crate::attrs::{GetAttr, NestedAttrGSW};
use syn::Attribute;

impl GetSetWithAttr {
    const BASEPATH: &'static str = "gsw";

    /// update the inner attribute
    pub fn add_prefix(&mut self, attr: GetAttr) {
        self.inner = Some(attr);
    }

    // tries to extract the root attribute from a list of attributes
    pub fn extract(attrs: &[Attribute]) -> syn::Result<Self> {
        let mut root = Self::default();
        for attr in attrs {
            if attr.path().is_ident(Self::BASEPATH) {
                attr.parse_nested_meta(|meta| {
                    if let Ok(nested) = NestedAttrGSW::parse_nested(&meta) {
                        match nested {
                            NestedAttrGSW::Get(inner) => {
                                root.add_prefix(inner);
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
