/*
    Appellation: contained-derive <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! derive macros for the contained crate
extern crate proc_macro;
extern crate quote;
extern crate syn;

#[allow(dead_code)]
pub(crate) mod attrs;

pub(crate) mod impls {
    #[doc(inline)]
    pub use self::{gsw::*, wrapper::*};

    mod gsw;
    mod wrapper;
}

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

/// The [`Wrapper`] macro is designed for single-field structs, implementing additional methods
/// supporting interactions with the inner value
#[proc_macro_derive(Wrapper, attributes(wrap))]
pub fn wrapper(input: TokenStream) -> TokenStream {
    // Parse the inputs into the proper struct
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let res = impls::impl_wrapper(&ast);

    res.into()
}

/// The [`Get`] derive macros is designed to streamline the process of creating getter methods
/// for structs. Coupled with the custom attributes, one can toggle the generation of mutable
/// getters and define alternative method names for accessing the inner value.
#[proc_macro_derive(Get, attributes(gsw))]
pub fn get(input: TokenStream) -> TokenStream {
    // Parse the inputs into the proper struct
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let res = impls::impl_get(&ast);

    res.into()
}

/// The [`SetWith`] macros is used to generate setter methods for struct fields.
#[proc_macro_derive(SetWith, attributes(gsw))]
pub fn set(input: TokenStream) -> TokenStream {
    // Parse the inputs into the proper struct
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let res = impls::impl_wrapper(&ast);

    res.into()
}
