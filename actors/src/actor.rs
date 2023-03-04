/*
    Appellation: actor <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{
    turing::{Move, Symbolic, Tape},
    State, States,
};

use serde::{Deserialize, Serialize};

pub trait Scope<S: Symbolic> {
    fn insert(&mut self, elem: S);
    fn position(&self) -> usize;
    fn set_position(&mut self, index: usize);
    fn set_state(&mut self, state: State<States>);
    fn set_symbol(&mut self, elem: S);
    /// [Move::Left] inserts a new element at the start of the tape if the current position is 0
    /// [Move::Right] inserts a new element at the end of the tape if the current position equals the total number of cells
    /// [Move::Stay] does nothing
    fn shift(&mut self, shift: Move, elem: S) {
        let index = self.position();
        match shift {
            // If the current position is 0, insert a new element at the top of the vector
            Move::Left if self.position() == 0 => {
                self.insert(elem);
            }
            Move::Left => {
                self.set_position(index - 1);
            }
            Move::Right => {
                self.set_position(index + 1);

                if self.position() == self.tape().len() {
                    self.insert(elem);
                }
            }
            Move::Stay => {}
        }
    }
    fn state(&self) -> &State<States>;
    fn scope(&self) -> &S {
        self.tape()
            .get(self.position())
            .expect("Index is out of bounds...")
    }
    fn tape(&self) -> &Tape<S>;

}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Operator<S: Symbolic = String>(usize, State<States>, Tape<S>);

impl<S: Symbolic> Operator<S> {
    pub fn new(index: usize, state: State<States>, tape: Tape<S>) -> Self {
        Self(index % tape.len(), state, tape)
    }
    pub fn norm(tape: Tape<S>) -> Self {
        Self::new(0, Default::default(), tape)
    }
    pub fn std(tape: Tape<S>) -> Self {
        Self::new(tape.len() - 1, Default::default(), tape)
    }
}

impl<S: Symbolic> Scope<S> for Operator<S> {
    fn insert(&mut self, elem: S) {
        self.2.insert(self.position(), elem);
    }

    fn position(&self) -> usize {
        self.0
    }

    fn set_position(&mut self, index: usize) {
        self.0 = index;
    }
    fn set_state(&mut self, state: State<States>) {
        self.1 = state;
    }
    fn set_symbol(&mut self, elem: S) {
        self.2.set(self.position(), elem);
    }

    fn state(&self) -> &State<States> {
        &self.1
    }

    fn tape(&self) -> &Tape<S> {
        &self.2
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
