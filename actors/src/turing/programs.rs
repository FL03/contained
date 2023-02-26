/*
    Appellation: programs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{Head, Instruction};
use crate::{Resultant, State, States, Symbolic};

use scsys::prelude::Stateful;
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
    /// Given some [Head], find the coresponding [Instruction]
    fn get(&self, head: Head<S>) -> Resultant<&Instruction<S>> {
        if self.final_state().clone().state() < head.state().clone().state() {
            Err("The provided head is greater than the final state...".to_string())
        } else {
            match self
                .instructions()
                .iter()
                .find(|inst: &&Instruction<S>| inst.head == head)
            {
                None => Err("Failed to find instructions for the provided head...".to_string()),
                Some(v) => Ok(v),
            }
        }
    }
    /// Insert a new [Instruction] set into the program
    fn insert(&mut self, inst: Instruction<S>) -> Resultant<Option<Instruction<S>>> {
        if inst.head.state() == &State::from(&States::invalid()) {
            return Err("Set error: Instruction cannot have 0 state in head...".to_string());
        }
        if !self.alphabet().contains(inst.head.symbol())
            || !self.alphabet().contains(inst.tail.symbol())
        {
            return Err(
                "The provided instruction set fails to be represented within the alphabet..."
                    .to_string(),
            );
        }
        if self.final_state().clone().state() < inst.head.state().clone().state()
            || self.final_state().clone().state() < inst.tail.state().clone().state()
        {
            return Err("Instructions have states greater than the ones availible...".to_string());
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
        let s: i64 = final_state.clone().state().into();
        let capacity = alphabet.len() * s as usize;
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
    use crate::{State, States};

    #[test]
    fn test_program() {
        let inst = Instruction::from((
            State::from(&States::valid()),
            "a",
            State::from(&States::valid()),
            "b",
            Move::Right,
        ));
        let alphabet = vec!["a", "b", "c"];
        let mut program = Program::new(alphabet, State::from(&States::invalid()));

        assert!(program.insert(inst.clone()).is_ok());
        assert!(program.get(inst.head).is_ok())
    }
}
