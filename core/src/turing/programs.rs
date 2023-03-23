/*
    Appellation: programs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::instructions::{Head, Instruction};
use crate::states::{State, Stateful};
use crate::{Alphabet, Include, Insert, Symbolic};
use serde::{Deserialize, Serialize};
use std::mem::replace;

pub trait Contract<S: Symbolic>:
    Clone + Include<Instruction<S>> + Insert<usize, Instruction<S>>
{
    fn alphabet(&self) -> Box<dyn Alphabet<S>>;
    fn final_state(&self) -> State;

    /// Given some [Head], find the coresponding [Instruction]
    fn get(&self, head: Head<S>) -> Option<&Instruction<S>> {
        // TODO: Reimplement the checks for getting a head value
        if head.state() > self.final_state() {
            return None;
        }
        self.instructions()
            .iter()
            .find(|inst: &&Instruction<S>| inst.head() == head)
    }
    /// Try to insert a new [Instruction] into the program; if the instruction is invalid, return None
    /// Otherwise, return the previous instruction at the same [Head] if it exists
    fn insert(&mut self, inst: Instruction<S>) -> Option<Instruction<S>> {
        // TODO: Reimplement the checks for insertion
        if inst.head().state() == State::Invalid {
            return None;
        }
        if self.final_state() < inst.head().state() || self.final_state() < inst.tail().state() {
            return None;
        }
        if !self.alphabet().in_alphabet(&inst.head().symbol())
            || !self.alphabet().in_alphabet(&inst.tail().symbol())
        {
            return None;
        }

        match self
            .instructions()
            .iter()
            .position(|cand: &Instruction<S>| cand.head() == inst.head())
        {
            Some(index) => Some(replace(&mut self.instructions_mut()[index], inst)),
            None => {
                self.instructions_mut().push(inst.clone());
                Some(inst)
            }
        }
    }

    fn instructions(&self) -> &Vec<Instruction<S>>;
    fn instructions_mut(&mut self) -> &mut Vec<Instruction<S>>;
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Program<S: Symbolic> {
    pub alphabet: Vec<S>,
    instructions: Vec<Instruction<S>>,
    final_state: State,
}

impl<S: Symbolic> Program<S> {
    pub fn new(alphabet: impl IntoIterator<Item = S>, final_state: State) -> Self {
        let alphabet = Vec::from_iter(alphabet);
        let s: i64 = final_state.into();
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
    /// Returns an owned instance of the current [Instruction] set
    pub fn instructions(&self) -> &Vec<Instruction<S>> {
        &self.instructions
    }
    /// Returns an owned instance of the final state
    pub fn final_state(&self) -> &State {
        &self.final_state
    }
    /// Given some [Head], find the coresponding [Instruction]
    pub fn get(&self, head: Head<S>) -> Option<&Instruction<S>> {
        if head.state() > *self.final_state() {
            return None;
        }
        self.instructions()
            .iter()
            .find(|inst: &&Instruction<S>| inst.head() == head)
    }
    /// Try to insert a new [Instruction] into the program; if the instruction is invalid, return None
    /// Otherwise, return the previous instruction at the same [Head] if it exists
    pub fn insert(&mut self, inst: Instruction<S>) -> Option<Instruction<S>> {
        // TODO: Reimplement the checks for insertion
        if inst.head().state() == State::Invalid {
            return None;
        }
        if *self.final_state() < inst.head().state() || *self.final_state() < inst.tail().state() {
            return None;
        }
        if !self.alphabet().in_alphabet(&inst.head().symbol())
            || !self.alphabet().in_alphabet(&inst.tail().symbol())
        {
            return None;
        }

        match self
            .instructions()
            .iter()
            .position(|cand: &Instruction<S>| cand.head() == inst.head())
        {
            Some(index) => Some(replace(&mut self.instructions[index], inst)),
            None => {
                self.instructions.push(inst.clone());
                Some(inst)
            }
        }
    }
}

impl<S: Symbolic> Alphabet<S> for Program<S> {
    fn in_alphabet(&self, symbol: &S) -> bool {
        self.alphabet.in_alphabet(symbol)
    }
    fn default_symbol(&self) -> S {
        self.alphabet.default_symbol()
    }
}

impl<S: Symbolic> Extend<Instruction<S>> for Program<S> {
    fn extend<T: IntoIterator<Item = Instruction<S>>>(&mut self, iter: T) {
        for i in iter {
            self.insert(i);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::turing::instructions::Move;

    #[test]
    fn test_program() {
        let inst = Instruction::from((State::valid(), "a", State::valid(), "b", Move::Right));
        let mut program = Program::new(vec!["a", "b", "c"], State::invalid());

        assert!(program.insert(inst.clone()).is_some());
        assert!(program.get(inst.head().clone()).is_some())
    }
}
