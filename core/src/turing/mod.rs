/*
    Appellation: turing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{machine::*, operator::*, programs::*, tape::*};

pub mod instructions;

pub(crate) mod machine;
pub(crate) mod operator;
pub(crate) mod programs;
pub(crate) mod tape;

use crate::{Scope, Symbolic};

/// [Turing] describes a programmable Turing machine
pub trait Turing<S: Symbolic> {
    type Error;
    type Scope: Scope<S>;

    /// [Turing::execute]
    fn execute(&mut self) -> Result<&Self, Self::Error>;
    /// [Turing::execute_once]
    fn execute_once(&mut self) -> Result<&Self, Self::Error>;
    /// [Turing::execute_until]
    fn execute_until(&mut self, until: impl Fn(&Self::Scope) -> bool)
        -> Result<&Self, Self::Error>;
    /// [Turing::translate] returns the mutated [Tape] after updating the [Turing::Scope] and finally invoking [Turing::execute]
    fn translate(&mut self, tape: Tape<S>) -> Result<Tape<S>, Self::Error>;
}
