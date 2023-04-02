/*
    Appellation: turing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{driver::*, platform::*, programs::*, tape::*};

pub mod instructions;

mod driver;
mod platform;
mod programs;
mod tape;

use crate::{Scope, Translate};
use std::collections::{BTreeSet, HashSet};

/// [Alphabet] describes an immutable set of [Symbolic] elements
pub trait Alphabet<S: Symbolic> {
    /// [Alphabet::default_symbol]
    fn default_symbol(&self) -> S {
        Default::default()
    }
    /// Returns true if the symbol is in the alphabet
    fn is_viable(&self, symbol: &S) -> bool;
}

impl<S: Symbolic> Alphabet<S> for Vec<S> {
    fn is_viable(&self, symbol: &S) -> bool {
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
    fn is_viable(&self, symbol: &S) -> bool {
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
    fn is_viable(&self, symbol: &S) -> bool {
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

/// [Turing] describes a programmable Turing machine
pub trait Turing<S: Symbolic>: Translate<S> {
    type Scope: Scope<S>;

    /// [Turing::execute]
    fn execute(&mut self) -> Result<&Self, Self::Error>;
    /// [Turing::execute_once]
    fn execute_once(&mut self) -> Result<&Self, Self::Error>;
    /// [Turing::execute_until]
    fn execute_until(&mut self, until: impl Fn(&Self::Scope) -> bool)
        -> Result<&Self, Self::Error>;
}
