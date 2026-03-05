/*
    Appellation: wrapper <module>
    Created At: 2026.01.20:14:20:08
    Contrib: @FL03
*/
mod impl_display_attr;
mod impl_nested_attr;
mod impl_wrapper_attr;

use syn::Ident;

/// AST for the root attribute
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WrapperAttr {
    pub inner: Option<DisplayAttr>,
}

/// [`NestedAttr`] is an enumeration of various nested attributes the crate recognizes.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NestedAttr {
    Inner(DisplayAttr),
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DisplayAttr {
    pub format: Option<Ident>,
}
