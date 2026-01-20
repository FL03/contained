/*
    Appellation: lib <module>
    Created At: 2026.01.20:14:52:56
    Contrib: @FL03
*/
//! the core modules supporting the `contained` crate focused on establishing foundational
//! primitives and utilities for getter, setters, and wrappers.
#![crate_type = "lib"]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "alloc", feature = "nightly"), feature(allocator_api))]
// external crates
#[cfg(feature = "alloc")]
extern crate alloc;
// macros
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
    #[macro_use]
    pub mod format;
    #[macro_use]
    pub mod wrapper;
}
// modules
pub mod error;

pub mod traits {
    //! core traits and interfaces for wrappers and their operations, formatting, etc.
    #[doc(inline)]
    pub use self::{get::*, wrapper::*};

    mod get;
    mod wrapper;
}
// re-exports
#[doc(inline)]
pub use self::error::{Error, Result};
// prelude
#[doc(hidden)]
pub mod prelude {
    #[cfg(feature = "macros")]
    pub use crate::{fmt_wrapper, wrapper};
}
