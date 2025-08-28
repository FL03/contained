/*
    Appellation: contained-macros <library>
    Contributors: FL03 <jo3mccain@icloud.com>
*/
//! procedural macros for interacting with various wrappers
extern crate proc_macro;

pub(crate) mod impl_binary;

pub(crate) mod ast {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use self::{ops::*, wrapper::*};

    mod ops;
    #[allow(dead_code)]
    mod wrapper;
}

use crate::ast::WrapperOpsAst;
use proc_macro::TokenStream;
use syn::parse_macro_input;

/// A procedural macro for generating implementations for core binary operations on a wrapper
/// type
#[proc_macro]
pub fn binary_wrapper(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as WrapperOpsAst);
    let output = impl_binary::impl_wrapper_binary_ops(ast);
    output.into()
}
