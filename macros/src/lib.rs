/*
    Appellation: contained-macros <library>
    Contributors: FL03 <jo3mccain@icloud.com>
*/
//! procedural macros for interacting with various wrappers
extern crate proc_macro;

pub(crate) mod impl_binary;

use proc_macro::TokenStream;

/// Automatically generate the binary operator implementations for a wrapper type
#[proc_macro]
pub fn binary_wrapper(input: TokenStream) -> TokenStream {
    let output = impl_binary::wrapper_ops_impl(input);

    TokenStream::from(output)
}
