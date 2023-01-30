/*
    Appellation: programs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{Head, Instruction, Symbolic};
use crate::States;

use scsys::Result;
use std::mem::replace;

pub struct Program<S: Symbolic> {
    pub alphabet: Vec<S>,
    pub instructions: Vec<Instruction<S>>,
    pub final_state: States,
}

impl<S: Symbolic> Program<S> {
    pub fn new(alphabet: Vec<S>, final_state: States) -> Self {
        let s: i64 = final_state.clone().into();
        let capacity = alphabet.clone().len() * s as usize;
        let instructions = Vec::with_capacity(capacity);

        Self {
            alphabet,
            instructions,
            final_state,
        }
    }
    pub fn alphabet(&self) -> &Vec<S> {
        &self.alphabet
    }
    pub fn instructions(&self) -> &Vec<Instruction<S>> {
        &self.instructions
    }
    pub fn final_state(&self) -> &States {
        &self.final_state
    }
    pub fn get(&self, head: Head<S>) -> Result<&Instruction<S>> {
        if self.final_state() < head.state() {
            panic!("The provided head is greater than the final state...")
        } else {
            match self
                .instructions()
                .iter()
                .find(|inst: &&Instruction<S>| &inst.head == &head)
            {
                None => panic!("Failed to find instructions for the provided head..."),
                Some(v) => Ok(v),
            }
        }
    }
    pub fn insert(&mut self, inst: Instruction<S>) -> Result<Option<Instruction<S>>> {
        if inst.head.state() == &States::from(0) {
            panic!("Set error: Instruction cannot have 0 state in head...")
        }
        if !self.alphabet.contains(inst.head.symbol())
            || !self.alphabet.contains(inst.tail.symbol())
        {
            panic!("Program not ")
        }
        if self.final_state() < inst.head.state() || self.final_state() < inst.tail.state() {
            panic!("Instructions have states greater than the ones availible...")
        }
        let position = self
            .instructions
            .iter()
            .position(|cand: &Instruction<S>| cand.head == inst.head);

        match position {
            Some(index) => Ok(Some(replace(&mut self.instructions[index], inst))),
            None => {
                self.instructions.push(inst);
                Ok(None)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::turing::{Instruction, Move};

    #[test]
    fn test_program() {
        let inst = Instruction::from((States::Valid, "a", States::Valid, "b", Move::Right));
        let alphabet = vec!["a", "b", "c"];
        let mut program = Program::new(alphabet, 1.into());

        assert!(program.insert(inst.clone()).is_ok());
        assert!(program.get(inst.head).is_ok())
    }
}
