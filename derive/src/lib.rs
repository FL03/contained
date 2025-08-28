/*
    Appellation: contained-derive <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! derive macros for facilitating the creation of wrapper types

extern crate proc_macro;
extern crate quote;
extern crate syn;

#[allow(unused)]
pub(crate) mod attrs {
    pub use self::{display_attrs::*, nested::*, root::*};

    mod display_attrs;
    mod nested;
    mod root;
}

pub(crate) mod impls {
    #[doc(inline)]
    pub use self::wrapper::*;

    mod wrapper;
}
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

/// The [`Wrapper`] macro is designed for single-field structs, implementing additional methods
/// supporting interactions with the inner value
#[proc_macro_derive(Wrapper, attributes(wrapper))]
pub fn wrapper(input: TokenStream) -> TokenStream {
    // Parse the inputs into the proper struct
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let res = impls::impl_wrapper(&ast);

    res.into()
}
