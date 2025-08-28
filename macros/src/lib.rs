/*
    Appellation: contained-macros <library>
    Contributors: FL03 <jo3mccain@icloud.com>
*/
//! procedural macros for interacting with various wrappers
extern crate proc_macro;

pub(crate) mod impl_binary;

pub(crate) mod ast {
    #[doc(inline)]
    pub use self::ops::*;

    mod ops;
}

use proc_macro::TokenStream;

/// A procedural macro for generating implementations for core binary operations on a wrapper 
/// type
#[proc_macro]
pub fn binary_wrapper(input: TokenStream) -> TokenStream {
    let output = impl_binary::impl_wrapper_binary_ops(input);

    TokenStream::from(output)
}
