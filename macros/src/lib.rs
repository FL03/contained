/*
    Appellation: contained-macros <library>
    Contributors: FL03 <jo3mccain@icloud.com>
*/
//! procedural macros for interacting with various wrappers
extern crate proc_macro;

use proc_macro::TokenStream;

pub(crate) mod wrap;

///
#[proc_macro]
pub fn binary_wrapper(input: TokenStream) -> TokenStream {
    wrap::wrapper_ops_impl(input)
}
