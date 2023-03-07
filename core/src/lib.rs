/*
    Appellation: actors <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{primitives::*, scope::*, specs::*, utils::*};

pub(crate) mod primitives;
pub(crate) mod scope;
pub(crate) mod specs;
pub(crate) mod utils;

pub mod states;
pub mod turing;

/// [Alphabet] describes an immutable set of [Symbolic] elements
pub trait Alphabet<S: Symbolic>: Clone + IntoIterator<Item = S> {
    /// [Alphabet::alphabet]
    fn alphabet(self) -> Vec<S> {
        Vec::from_iter(self)
    }
    /// [Alphabet::default_symbol]
    fn default_symbol(&self) -> S {
        match self.clone().alphabet().first() {
            Some(v) => v.clone(),
            None => Default::default(),
        }
    }
}

impl<S: Symbolic> Alphabet<S> for Vec<S> {}

/// Simple trait for compatible symbols
pub trait Symbolic: Clone + Default + PartialEq + std::fmt::Debug + std::fmt::Display {}

impl Symbolic for char {}

impl Symbolic for &str {}

impl Symbolic for String {}
