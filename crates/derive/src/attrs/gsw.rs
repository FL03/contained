/*
    Appellation: gsw <module>
    Created At: 2026.01.20:13:04:36
    Contrib: @FL03
*/
mod impl_getter_attr;
mod impl_gsw_attr;
mod impl_nested_gsw_attr;

use syn::Ident;

/// The abstract syntax-tree for the `gsw` attribute
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GetSetWithAttr {
    pub inner: Option<GetAttr>,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NestedAttrGSW {
    Get(GetAttr),
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GetAttr {
    pub format: Option<Ident>,
    /// a toggle indicating whether the setter method should be mutable
    pub mutable: bool,
}
