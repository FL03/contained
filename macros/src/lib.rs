/*
    Appellation: contained-macros <library>
    Contributors: FL03 <jo3mccain@icloud.com>
*/
//! procedural macros for interacting with various wrappers
extern crate proc_macro;

pub(crate) mod wrap;

use proc_macro::TokenStream;

///
#[proc_macro]
pub fn binary_wrapper(input: TokenStream) -> TokenStream {
    let output = wrap::wrapper_ops_impl(input);

    TokenStream::from(output)
}
