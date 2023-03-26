/*
    Appellation: iter <instructions>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        This module contains the implementation of the [InstructionSet] trait.
*/
use super::*;
use crate::Symbolic;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Iter<S: Symbolic> {
    cursor: usize,
    instructions: Vec<Instruction<S>>,
}

impl<S: Symbolic> Iter<S> {
    pub fn new(instructions: Vec<Instruction<S>>) -> Self {
        Self {
            cursor: 0,
            instructions,
        }
    }
}

impl<S: Symbolic> Extend<Instruction<S>> for Iter<S> {
    fn extend<T: IntoIterator<Item = Instruction<S>>>(&mut self, iter: T) {
        self.instructions.extend(iter)
    }
}

impl<S: Symbolic> Iterator for Iter<S> {
    type Item = Instruction<S>;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = self.instructions.get(self.cursor).cloned();
        self.cursor += 1;
        instruction
    }
}
