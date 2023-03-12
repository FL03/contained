/*
    Appellation: instructions <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Turing machines accept instructions in the form of a five-tuple:
            (State, Symbol, State, Symbol, Move)
*/
pub use self::{head::*, instruction::*, moves::*, tail::*};

pub(crate) mod head;
pub(crate) mod instruction;
pub(crate) mod moves;
pub(crate) mod tail;

use crate::turing::Symbolic;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct InstructionSet<S: Symbolic>(Vec<Instruction<S>>);

impl<S: Symbolic> InstructionSet<S> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn push(&mut self, elem: Instruction<S>) {
        self.0.push(elem)
    }
}
