//! `contained` is a collection of utilities focused on facilitating the implementation of
//! so-called wrapper types as well as providing additional
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
// re-exports
#[doc(inline)]
pub use contained_core::*;
#[doc(inline)]
#[cfg(feature = "derive")]
pub use contained_derive::*;
#[doc(inline)]
#[cfg(feature = "macros")]
pub use contained_macros::*;
// prelude
#[doc(hidden)]
pub mod prelude {
    #[cfg(feature = "derive")]
    pub use contained_derive::*;
    #[cfg(feature = "macros")]
    pub use contained_macros::*;
}
