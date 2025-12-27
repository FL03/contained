/*
    Appellation: core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! this core components of the contained crate
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::upper_case_acronyms,
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]
// compile-time checks
#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error! { "Either the 'std' or 'alloc' feature must be enabled." }
// external crates
#[cfg(feature = "alloc")]
extern crate alloc;
// macros
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;

    #[macro_use]
    #[cfg(feature = "macros")]
    pub mod ext {
        #[macro_use]
        pub mod format;
        #[macro_use]
        pub mod wrapper;
    }
}
// modules
pub mod error;

pub mod traits {
    //! core traits and interfaces for wrappers and their operations, formatting, etc.
}
// re-exports
#[doc(inline)]
pub use self::{
    error::{Error, Result},
};
// prelude
#[doc(hidden)]
pub mod prelude {
    #[cfg(feature = "macros")]
    pub use crate::{fmt_wrapper, wrapper};
}
