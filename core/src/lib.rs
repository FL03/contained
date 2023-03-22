/*
    Appellation: actors <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{errors::*, primitives::*, scope::*, specs::*, utils::*};

pub(crate) mod errors;
pub(crate) mod primitives;
pub(crate) mod scope;
pub(crate) mod specs;
pub(crate) mod utils;

pub mod actors;
pub mod states;
pub mod turing;

use std::collections::{BTreeSet, HashSet};

/// [Alphabet] describes an immutable set of [Symbolic] elements
pub trait Alphabet<S: Symbolic> {
    /// [Alphabet::default_symbol]
    fn default_symbol(&self) -> S {
        Default::default()
    }
}

impl<S: Symbolic> Alphabet<S> for (S,) {
    fn default_symbol(&self) -> S {
        self.0.clone()
    }
}

impl<S: Symbolic> Alphabet<S> for Vec<S> {
    fn default_symbol(&self) -> S {
        if let Some(entry) = self.first() {
            entry.clone()
        } else {
            Default::default()
        }
    }
}

impl<S: Symbolic> Alphabet<S> for BTreeSet<S> {
    fn default_symbol(&self) -> S {
        if let Some(entry) = self.first() {
            entry.clone()
        } else {
            Default::default()
        }
    }
}

impl<S: Symbolic> Alphabet<S> for HashSet<S> {
    fn default_symbol(&self) -> S {
        if let Some(entry) = self.iter().next() {
            entry.clone()
        } else {
            Default::default()
        }
    }
}

/// Simple trait for compatible symbols
pub trait Symbolic: Clone + Default + Eq + Ord + std::fmt::Debug + std::fmt::Display {}

impl Symbolic for char {}

impl Symbolic for &str {}

impl Symbolic for String {}
