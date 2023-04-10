/*
    Appellation: turing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{exec::*, programs::*, states::*, tape::*};

pub mod instructions;
pub mod machine;

mod exec;
mod programs;
mod states;
mod tape;

use crate::{ArrayLike, Include, Insert};
use crate::states::Stateful;
use instructions::Move;
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

/// [Scope] describes the focus of the [crate::turing::Turing]
pub trait Scope<S: Symbolic>: Include<S> + Insert<usize, S> + Stateful<State> {
    /// [Scope::current] returns the current element of the [Scope] on the [Tape]
    fn current(&self) -> S {
        self.tape()
            .get(self.cursor())
            .expect("Index is out of bounds...")
            .clone()
    }
    /// [Scope::cursor] returns the current position of the [Scope] on the [Tape]
    fn cursor(&self) -> usize;
    /// [Scope::set_index] sets the current position of the [Scope] on the [Tape]
    fn set_index(&mut self, index: usize);
    /// [Scope::set_symbol] sets the current element of the [Scope] on the [Tape]
    fn set_symbol(&mut self, elem: S);
    /// [Move::Left] inserts a new element at the start of the tape if the current position is 0
    /// [Move::Right] inserts a new element at the end of the tape if the current position equals the total number of cells
    /// [Move::Stay] does nothing
    fn shift(&mut self, shift: Move, elem: S) {
        let index = self.cursor();

        match shift {
            // If the current position is 0, insert a new element at the top of the vector
            Move::Left if self.cursor() == 0 => {
                self.include(elem);
            }
            Move::Left => {
                self.set_index(index - 1);
            }
            Move::Right => {
                self.set_index(index + 1);

                if self.cursor() == self.tape().len() {
                    self.include(elem);
                }
            }
            Move::Stay => {}
        }
    }
    /// [Scope::tape] returns the [Tape] of the [Scope]
    fn tape(&self) -> Tape<S>;
}

/// Simple trait for compatible symbols
pub trait Symbolic:
    Clone + Default + Eq + Ord + std::fmt::Debug + std::fmt::Display + std::hash::Hash
{
}

impl Symbolic for char {}

impl Symbolic for &str {}

impl Symbolic for String {}

/// [Translate] is a trait that allows for the translation of a machine's memory
pub trait Translate<S: Symbolic> {
    type Error;

    fn translate(&mut self, tape: Tape<S>) -> Result<Tape<S>, Self::Error>;
}

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

/// [With] describes a simple means of concating several objects together
pub trait With<T> {
    /// [With::Output] must be a superposition of self and T
    type Output;

    /// [With::with] accepts an owned instance of the given type and returns a [With::Output] instance
    fn with(&self, other: &T) -> Self::Output;
}

/// [TryWith] is a trait that describes a means of trying to concate several objects together
pub trait TryWith<T> {
    type Output;
    type Error;

    fn try_with(&self, other: &T) -> Result<Self::Output, Self::Error>;
}
