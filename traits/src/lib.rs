/*
    Appellation: container-traits <library>
    Created At: 2025.12.26:16:43:42
    Contrib: @FL03
*/
//! Traits and interfaces for establishing a sound foundation for containers and their elements.
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::should_implement_trait,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]
// compiler check
#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error! { "either the \"std\" or \"alloc\" feature must be enabled" }
// macros
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}
// external crates
#[cfg(feature = "alloc")]
extern crate alloc;
// modules
pub mod container;
pub mod key_value;
pub mod store;

pub mod ops {
    //! This module provides various operations traits and implementations for musical concepts
    #[doc(inline)]
    pub use self::{apply::*, transform::*};

    mod apply;
    mod transform;
}
// re-exports
#[doc(inline)]
pub use self::{container::*, ops::*, store::*};
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use crate::container::*;
    pub use crate::key_value::*;
    pub use crate::ops::*;
    pub use crate::store::*;
}
