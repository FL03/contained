/*
    Appellation: core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! this core components of the contained crate
#![allow(
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error! {
    "Either the 'std' or 'alloc' feature must be enabled."
}

#[cfg(feature = "alloc")]
extern crate alloc;

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

pub mod error;
// re-exports
#[doc(inline)]
pub use self::{
    error::{Error, Result},
    traits::prelude::*,
};
#[doc(inline)]
pub use contained_traits as traits;
// prelude
#[doc(hidden)]
pub mod prelude {
    #[cfg(feature = "macros")]
    pub use crate::{fmt_wrapper, wrapper};
}
