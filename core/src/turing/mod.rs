/*
    Appellation: turing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{machine::*, operator::*, programs::*};

pub mod instructions;
pub mod tapes;

pub(crate) mod machine;
pub(crate) mod operator;
pub(crate) mod programs;

use crate::{Scope, Symbolic};
use tapes::{Tape, Tapes};

pub trait Executable<T> {
    type Output;

    /// [Executable::execute]
    fn execute(&mut self) -> Self::Output;
    /// [Executable::execute_once]
    fn execute_once(&mut self) -> Self::Output;
    /// [Executable::execute_until]
    fn execute_until(&mut self, until: impl Fn(&T) -> bool) -> Self::Output;
}

/// [Turing] describes a programmable Turing machine
pub trait Turing<S: Symbolic> {
    type Error;
    type Scope: Clone + Scope<S>;

    /// [Turing::execute]
    fn execute(&mut self) -> Result<&Self, Self::Error>;
    /// [Turing::execute_once]
    fn execute_once(&mut self) -> Result<&Self, Self::Error>;
    /// [Turing::execute_until]
    fn execute_until(&mut self, until: impl Fn(&Self::Scope) -> bool)
        -> Result<&Self, Self::Error>;
    /// [Turing::translate] returns the mutated [Tape] after updating the [Turing::Scope] and finally invoking [Turing::execute]
    fn translate(&mut self, tape: Tapes<S>) -> Result<Tape<S>, Self::Error>;
}
