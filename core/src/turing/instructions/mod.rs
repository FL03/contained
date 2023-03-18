/*
    Appellation: instructions <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Turing machines accept instructions in the form of a five-tuple:
            (State, Symbol, State, Symbol, Move)
*/
pub use self::{head::*, instruction::*, iter::*, moves::*, tail::*};

pub(crate) mod head;
pub(crate) mod instruction;
pub(crate) mod iter;
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
    pub fn drain(&mut self, range: std::ops::Range<usize>) -> std::vec::Drain<Instruction<S>> {
        self.0.drain(range)
    }
    pub fn get(&self, index: usize) -> Option<&Instruction<S>> {
        self.0.get(index)
    }
    pub fn push(&mut self, elem: Instruction<S>) {
        self.0.push(elem)
    }
    pub fn set(&mut self, index: usize, elem: Instruction<S>) {
        self.0[index] = elem;
    }
}
