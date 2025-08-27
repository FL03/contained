/*
    Appellation: contained <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # contained
//!
//! Welcome to `contained`! A library focused on providing useful abstractions, macros, and
//! utilities for handling so-called wrapper types. In short, a wrapper type is any implemented
//! object capable of using `#[repr(transparent)]`.
#![allow(
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]

pub use contained_core::*;

#[cfg(feature = "derive")]
pub use contained_derive::*;
#[cfg(feature = "macros")]
pub use contained_macros::*;

#[allow(unused_imports)]
pub mod prelude {
    pub use contained_core::prelude::*;
    #[cfg(feature = "derive")]
    pub use contained_derive::*;
    #[cfg(feature = "macros")]
    pub use contained_macros::*;
}
