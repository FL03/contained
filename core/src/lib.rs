/*
    Appellation: actors <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{errors::*, primitives::*, specs::*, utils::*};

pub(crate) mod errors;
pub(crate) mod primitives;
pub(crate) mod specs;
pub(crate) mod utils;

pub mod actors;
pub mod states;
pub mod turing;

use std::collections::{BTreeSet, HashSet};

/// [Alphabet] describes an immutable set of [Symbolic] elements
pub trait Alphabet<S: Symbolic> {
    fn in_alphabet(&self, symbol: &S) -> bool;
    /// [Alphabet::default_symbol]
    fn default_symbol(&self) -> S {
        Default::default()
    }
}

impl<S: Symbolic> Alphabet<S> for Vec<S> {
    fn in_alphabet(&self, symbol: &S) -> bool {
        self.contains(symbol)
    }

    fn default_symbol(&self) -> S {
        if let Some(entry) = self.first() {
            entry.clone()
        } else {
            Default::default()
        }
    }
}

impl<S: Symbolic> Alphabet<S> for BTreeSet<S> {
    fn in_alphabet(&self, symbol: &S) -> bool {
        self.contains(symbol)
    }
    fn default_symbol(&self) -> S {
        if let Some(entry) = self.first() {
            entry.clone()
        } else {
            Default::default()
        }
    }
}

impl<S: Symbolic> Alphabet<S> for HashSet<S> {
    fn in_alphabet(&self, symbol: &S) -> bool {
        self.contains(symbol)
    }

    fn default_symbol(&self) -> S {
        if let Some(entry) = self.iter().next() {
            entry.clone()
        } else {
            Default::default()
        }
    }
}

/// Simple trait for compatible symbols
pub trait Symbolic:
    Clone + Default + Eq + Ord + std::fmt::Debug + std::fmt::Display + std::hash::Hash
{
}

impl Symbolic for char {}

impl Symbolic for &str {}

impl Symbolic for String {}
