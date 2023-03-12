/*
    Appellation: programs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::states::{State, Stateful, States};
use crate::turing::{Head, Instruction};
use crate::{Alphabet, Extend, Resultant, Symbolic};
use serde::{Deserialize, Serialize};
use std::mem::replace;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Program<S: Symbolic> {
    alphabet: Vec<S>,
    instructions: Vec<Instruction<S>>,
    final_state: State<States>,
}

impl<S: Symbolic> Program<S> {
    pub fn new(alphabet: Vec<S>, final_state: State) -> Self {
        let s: i64 = (*final_state.state()).into();
        let capacity = alphabet.len() * s as usize;
        let instructions = Vec::with_capacity(capacity);

        Self {
            alphabet,
            instructions,
            final_state,
        }
    }
    /// Returns an owned instance of the current program's alphabet
    pub fn alphabet(&self) -> &Vec<S> {
        &self.alphabet
    }
    pub fn default_symbol(&self) -> S {
        self.alphabet.default_symbol()
    }
    /// Returns an owned instance of the current [Instruction] set
    pub fn instructions(&self) -> &Vec<Instruction<S>> {
        &self.instructions
    }
    /// Returns an owned instance of the final state
    pub fn final_state(&self) -> &State {
        &self.final_state
    }
    /// Given some [Head], find the coresponding [Instruction]
    pub fn get(&self, head: Head<S>) -> Resultant<&Instruction<S>> {
        if self.final_state().state() < head.state().state() {
            return Err("The provided head is greater than the final state...".into());
        }
        if let Some(v) = self
            .instructions()
            .iter()
            .find(|inst: &&Instruction<S>| inst.head().clone() == head)
        {
            return Ok(v);
        }
        Err("Failed to find instructions for the provided head...".into())
    }
    /// Insert a new [Instruction] set into the program
    pub fn insert(&mut self, inst: Instruction<S>) -> Resultant<Option<Instruction<S>>> {
        if inst.head().state() == State::from(States::invalid()) {
            return Err("Set error: Instruction cannot have 0 state in head...".into());
        }
        if !self.alphabet().contains(&inst.head().symbol())
            || !self.alphabet().contains(&inst.tail().symbol())
        {
            return Err(
                "The provided instruction set fails to be represented within the alphabet..."
                    .into(),
            );
        }
        if self.final_state().state() < inst.head().state().state()
            || self.final_state().state() < inst.tail().state().state()
        {
            return Err("Instructions have states greater than the ones availible...".into());
        }
        let position = self
            .instructions()
            .iter()
            .position(|cand: &Instruction<S>| cand.head() == inst.head());

        match position {
            Some(index) => Ok(Some(replace(&mut self.instructions[index], inst))),
            None => {
                self.instructions.push(inst);
                Ok(None)
            }
        }
    }
}

impl<S: Symbolic> Extend<Instruction<S>> for Program<S> {
    type Output = Resultant;

    fn extend<T: IntoIterator<Item = Instruction<S>>>(&mut self, iter: T) -> Self::Output {
        for i in iter {
            self.insert(i)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::states::{State, States};
    use crate::turing::{Instruction, Move};

    #[test]
    fn test_program() {
        let inst = Instruction::from((
            State::from(States::valid()),
            "a",
            State::from(States::valid()),
            "b",
            Move::Right,
        ));
        let alphabet = vec!["a", "b", "c"];
        let mut program = Program::new(alphabet, State::from(States::invalid()));

        assert!(program.insert(inst.clone()).is_ok());
        assert!(program.get(inst.head().clone()).is_ok())
    }
}
