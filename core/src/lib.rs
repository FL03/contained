/*
    Appellation: actors <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{alphabet::*, primitives::*, scope::*, specs::*, utils::*};

pub(crate) mod alphabet;
pub(crate) mod primitives;
pub(crate) mod scope;
pub(crate) mod specs;
pub(crate) mod utils;

pub mod states;
pub mod turing;

/// Simple trait for compatible symbols
pub trait Symbolic: Clone + Default + PartialEq + std::fmt::Debug + std::fmt::Display {}

impl Symbolic for char {}

impl Symbolic for &str {}

impl Symbolic for String {}
