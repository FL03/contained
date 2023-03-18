/*
    Appellation: iter <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: implements an explicit iterator for a set of instructions
*/
use super::{Instruction, InstructionSet};
use crate::turing::Symbolic;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Iter<S: Symbolic> {
    index: usize,
    iter: InstructionSet<S>
}

impl<S: Symbolic> Iter<S> {
    pub fn new(iter: InstructionSet<S>) -> Self {
        Self {
            index: 0,
            iter
        }
    }
}

impl<S: Symbolic> Iterator for Iter<S> {
    type Item = Instruction<S>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        if let Some(item) = self.iter.get(index) {
            Some(item.clone())
        } else {
            None
        }
    }
}
