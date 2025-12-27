/*
    Appellation: contained-macros <library>
    Contributors: FL03 <jo3mccain@icloud.com>
*/
//! procedural macros for interacting with various wrappers
extern crate proc_macro;

mod ast;
mod impls;

use crate::ast::WrapperImpls;
use proc_macro::TokenStream;
use syn::parse_macro_input;

/// The [`binary_wrapper!`] macro generates implementations for the core binary operations
/// onto a generic wrapper type. It supports both tuple structs and structs with named fields.
///
/// ```rust
/// extern crate contained_macros as macros;
///
/// pub struct Wrapper<T>(pub T);
///
/// macros::binary_wrapper! {
///     impl Wrapper {
///         Add.add,
///         Sub.sub,
///         Mul.mul,
///         Div.div,
///         Rem.rem,
///     }
/// }
/// ```
///
/// or, for transparent structs with a named field:
///
/// ```rust
/// extern crate contained_macros as macros;
///
/// pub struct Wrapper<T> {
///     pub field: T,
/// }
///
/// macros::binary_wrapper! {
///     impl Wrapper.field {
///         Add.add,
///         Sub.sub,
///         Mul.mul,
///         Div.div,
///         Rem.rem,
///     }
/// }
/// ```
#[proc_macro]
pub fn binary_wrapper(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as WrapperImpls);
    let output = impls::impl_wrapper_binary_ops(ast);
    output.into()
}

/// The [`unary_wrapper!`] macro generates implementations for the core unary operations
/// onto a generic wrapper type. It supports both tuple structs and structs with named fields.
///
/// ```rust
/// extern crate contained_macros as macros;
///
/// pub struct Wrapper<T>(pub T);
///
/// macros::unary_wrapper! {
///     impl Wrapper {
///         Neg.neg,
///         Not.not,
///     }
/// }
/// ```
///
/// or, for transparent structs with a named field:
///
/// ```rust
/// extern crate contained_macros as macros;
///
/// pub struct Wrapper<T> {
///     pub field: T,
/// }
///
/// macros::unary_wrapper! {
///     impl Wrapper.field {
///         Neg.neg,
///         Not.not,
///     }
/// }
/// ```
#[proc_macro]
pub fn unary_wrapper(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as WrapperImpls);
    let output = impls::impl_wrapper_unary_ops(ast);
    output.into()
}
