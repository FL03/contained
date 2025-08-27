/*
    appellation: root <module>
    authors: @FL03
*/
use crate::attrs::{DisplayAttr, NestedAttr};
use syn::Attribute;

// AST for the scsys attribute
#[derive(Debug, Default)]
pub struct ContainedAttr {
    pub display: Option<DisplayAttr>,
}

impl ContainedAttr {
    pub fn set_display(&mut self, display: DisplayAttr) {
        self.display = Some(display);
    }

    // tries to extract the scsys attribute from a list of attributes
    pub fn extract(attrs: &[Attribute]) -> syn::Result<Self> {
        let mut scsys = Self::default();
        for attr in attrs {
            if attr.path().is_ident("contained") {
                attr.parse_nested_meta(|meta| {
                    if let Ok(nested) = NestedAttr::parse_nested(&meta) {
                        match nested {
                            NestedAttr::Display(inner) => {
                                scsys.set_display(inner);
                                return Ok(());
                            }
                        }
                    }
                    Err(meta.error("unrecognized attribute"))
                })?;
            }
        }
        Ok(scsys)
    }
}
