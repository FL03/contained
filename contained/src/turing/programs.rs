/*
    Appellation: programs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{Head, Instruction, Symbolic};
use crate::{Resultant, State};

use serde::{Deserialize, Serialize};
use std::mem::replace;

pub trait Programatic<S: Symbolic> {
    ///
    fn alphabet(&self) -> &Vec<S>;
    ///
    fn instructions(&self) -> &Vec<Instruction<S>>;
    ///
    fn mut_instructions(&mut self) -> &mut Vec<Instruction<S>>;
    ///
    fn final_state(&self) -> &State;
    ///
    fn get(&self, head: Head<S>) -> Resultant<&Instruction<S>> {
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
    ///
    fn insert(&mut self, inst: Instruction<S>) -> Resultant<Option<Instruction<S>>> {
        if inst.head.state() == &State::from(0) {
            return Err(format!(
                "Set error: Instruction cannot have 0 state in head..."
            ));
        }
        if !self.alphabet().contains(inst.head.symbol())
            || !self.alphabet().contains(inst.tail.symbol())
        {
            return Err(format!(
                "The provided instruction set fails to be represented within the alphabet..."
            ));
        }
        if self.final_state() < inst.head.state() || self.final_state() < inst.tail.state() {
            return Err(format!(
                "Instructions have states greater than the ones availible..."
            ));
        }
        let position = self
            .instructions()
            .iter()
            .position(|cand: &Instruction<S>| cand.head == inst.head);

        match position {
            Some(index) => Ok(Some(replace(&mut self.mut_instructions()[index], inst))),
            None => {
                self.mut_instructions().push(inst);
                Ok(None)
            }
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Program<S: Symbolic> {
    pub alphabet: Vec<S>,
    pub instructions: Vec<Instruction<S>>,
    pub final_state: State,
}

impl<S: Symbolic> Program<S> {
    pub fn new(alphabet: Vec<S>, final_state: State) -> Self {
        let s: i64 = final_state.clone().into();
        let capacity = alphabet.clone().len() * s as usize;
        let instructions = Vec::with_capacity(capacity);

        Self {
            alphabet,
            instructions,
            final_state,
        }
    }
}

impl<S: Symbolic> Programatic<S> for Program<S> {
    fn alphabet(&self) -> &Vec<S> {
        &self.alphabet
    }

    fn instructions(&self) -> &Vec<Instruction<S>> {
        &self.instructions
    }

    fn mut_instructions(&mut self) -> &mut Vec<Instruction<S>> {
        &mut self.instructions
    }

    fn final_state(&self) -> &State {
        &self.final_state
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::turing::{Instruction, Move};

    #[test]
    fn test_program() {
        let inst = Instruction::from((1.into(), "a", 2.into(), "b", Move::Right));
        let alphabet = vec!["a", "b", "c"];
        let mut program = Program::new(alphabet, 1.into());

        assert!(program.insert(inst.clone()).is_ok());
        assert!(program.get(inst.head).is_ok())
    }
}