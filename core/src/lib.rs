/*
    Appellation: actors <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{errors::*, primitives::*, specs::*, states::*, utils::*};

pub(crate) mod errors;
pub(crate) mod primitives;
pub(crate) mod specs;
pub(crate) mod states;
pub(crate) mod utils;

pub mod actors;
pub mod turing;

use turing::instructions::Instruction;

use predicates::Predicate;
use std::collections::{BTreeSet, HashSet};

/// [Executable] is a trait that allows for the execution of a program.
pub trait Executable<S: Symbolic>: Clone + Alphabet<S> + Iterator<Item = Instruction<S>> {
    type Driver: Scope<S>;
    type Error;

    fn execute(&mut self, driver: &mut Self::Driver) -> Result<Self::Driver, Self::Error> {
        // Get the default symbol
        let default_symbol = self.clone().default_symbol();
        // Get the next instruction
        while let Some(instruction) = self.next() {
            let tail = instruction.clone().tail();
            // Update the current state
            driver.update_state(tail.state());
            // Update the tape
            driver.set_symbol(tail.symbol());
            // Update the index; adjusts the index according to the direction
            driver.shift(tail.action(), default_symbol.clone());
        }
        // Return the actor
        Ok(driver.clone())
    }
    fn execute_once(&mut self, driver: &mut Self::Driver) -> Result<Self::Driver, Self::Error> {
        // Get the default symbol
        let default_symbol = self.clone().default_symbol();
        // Get the next instruction
        if let Some(instruction) = self.next() {
            let tail = instruction.clone().tail();
            // Update the current state
            driver.update_state(tail.state());
            // Update the tape
            driver.set_symbol(tail.symbol());
            // Update the index; adjusts the index according to the direction
            driver.shift(tail.action(), default_symbol.clone());
        }
        // Return the actor
        Ok(driver.clone())
    }
    fn execute_until(
        &mut self,
        driver: &mut Self::Driver,
        until: impl Predicate<Self::Driver>,
    ) -> Result<Self::Driver, Self::Error> {
        // Get the default symbol
        let default_symbol = self.clone().default_symbol();
        // Get the next instruction
        while let Some(instruction) = self.next() {
            let tail = instruction.clone().tail();
            // Update the current state
            driver.update_state(tail.state());
            // Update the tape
            driver.set_symbol(tail.symbol());
            // Update the index; adjusts the index according to the direction
            driver.shift(tail.action(), default_symbol.clone());
            if until.eval(driver) {
                break;
            }
        }
        // Return the actor
        Ok(driver.clone())
    }
}

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
