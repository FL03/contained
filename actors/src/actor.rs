/*
    Appellation: actor <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{State, States, turing::{Move, Symbolic, Tape}};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Operator<S: Symbolic>(usize, State<States>, Tape<S>);

impl<S: Symbolic> Operator<S> {
    pub fn new(index: usize, state: State<States>, tape: Tape<S>) -> Self {
        Self(index % tape.len(), state, tape)
    }
    pub fn insert(&mut self, elem: S) {
        self.2.insert(self.position(), elem);
    }
    pub fn position(&self) -> usize {
        self.0
    }
    pub fn set_symbol(&mut self, elem: S) {
        self.2.set(self.position(), elem)
    }
    /// [Move::Left] inserts a new element at the start of the tape if the current position is 0
    /// [Move::Right] inserts a new element at the end of the tape if the current position equals the total number of cells
    /// [Move::Stay] does nothing
    pub fn shift(&mut self, shift: Move, elem: S) {
        match shift {
            // If the current position is 0, insert a new element at the top of the vector
            Move::Left if self.position() == 0 => {
                self.insert(elem);
            }
            Move::Left => {
                self.0 -= 1;
            },
            Move::Right => {
                self.0 += 1;

                if self.position() == self.tape().len() {
                    self.insert(elem);
                }
            },
            Move::Stay => {}
        }
    }
    pub fn state(&self) -> &State<States> {
        &self.1
    }
    pub fn scope(&self) -> &S {
        self.tape().get(self.position()).expect("Index is out of bounds...")
    }
    pub fn tape(&self) -> &Tape<S> {
        &self.2
    }
    pub fn mut_tape(&mut self) -> &mut Tape<S> {
        &mut self.2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator() {
        let tape = Tape::new(["a", "b", "c"]);
        let mut actor = Operator(0, State::from(States::Valid), tape);
        actor.shift(Move::Left, "b");
        assert_eq!(actor.tape().clone(), Tape::new(["b", "a", "b", "c"]));
        for _ in 0..actor.tape().len() {
            actor.shift(Move::Right, "b");
        }
        assert_eq!(actor.tape().clone(), Tape::new(["b", "a", "b", "c", "b"]));
    }
}