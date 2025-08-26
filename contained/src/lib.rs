/*
    Appellation: contained <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # contained
//! 
//! Welcome to `contained`! A library focused on providing useful abstractions, macros, and 
//! utilities for handling so-called wrapper types. In short, a wrapper type is any implemented
//! object capable of using `#[repr(transparent)]`.

pub use contained_core::*;


pub mod prelude {
    pub use contained_core::prelude::*;
}
