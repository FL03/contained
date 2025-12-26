/*
    Appellation: seal <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! The public parts of this private module are used to create traits
//! that cannot be implemented outside of our own crate.  This way we
//! can feel free to extend those traits without worrying about it
//! being a breaking change for other implementations.
#![allow(dead_code, unused_macros)]

/// If this type is pub but not publicly reachable, third parties
/// can't name it and can't implement traits using it.
pub struct Seal;
/// The [`private`] macro injects a hidden, private method into a trait definition preventing
/// it from being implemented outside of the crate.
macro_rules! private {
    () => {
        /// This trait is private to implement; this method exists to make it
        /// impossible to implement outside the crate.
        #[doc(hidden)]
        fn __private__(&self) -> $crate::macros::seal::Seal;
    };
}
/// [`seal`] is a helper macro to streamline the implementation of _sealed_ traits.
macro_rules! seal {
    () => {
        fn __private__(&self) -> $crate::macros::seal::Seal {
            $crate::macros::seal::Seal
        }
    };
}
