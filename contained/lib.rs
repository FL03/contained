//! `contained` is a collection of macros, traits, and other useful primitives for defining,
//! managing, and interacting with so-called wrapper types. More specifically, any type capable
//! of deriving the `#[repr(transparent)]` attribute in Rust.
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]
// compile-time checks 
#[cfg(not(any(feature = "alloc", feature = "std")))]
compile_error! { "Either the 'alloc' or 'std' feature must be enabled for this crate to compile." }
// external crates
#[cfg(feature = "alloc")]
extern crate alloc;
// re-exports
pub use contained_core::*;
#[cfg(feature = "derive")]
pub use contained_derive::*;
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
