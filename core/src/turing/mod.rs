/*
    Appellation: turing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{driver::*, platform::*, programs::*, tape::*};

pub mod instructions;

pub(crate) mod driver;
pub(crate) mod platform;
pub(crate) mod programs;
pub(crate) mod tape;

use crate::{Scope, Symbolic, Translate};

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
