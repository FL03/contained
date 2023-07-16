/*
    Appellation: instructions <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Instructions
///
/// Turing machines accept instructions in the form of a five-tuple:
///    (State, Symbol, State, Symbol, Move)
use super::{Head, Move, Tail};
use crate::Symbolic;
use contained_core::prelude::State;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Instruction<S: Symbolic>(Head<S>, Tail<S>);

impl<S: Symbolic> Instruction<S> {
    pub fn new(head: Head<S>, tail: Tail<S>) -> Self {
        Self(head, tail)
    }
    pub fn head(&self) -> Head<S> {
        self.0.clone()
    }
    pub fn tail(&self) -> Tail<S> {
        self.1.clone()
    }
    pub fn update(&mut self, head: Head<S>, tail: Tail<S>) {
        self.0 = head;
        self.1 = tail;
    }
}

impl<S: Symbolic> From<(State, S, State, S, Move)> for Instruction<S> {
    fn from(value: (State, S, State, S, Move)) -> Self {
        let head = Head::new(value.0, value.1);
        let tail = Tail::new(value.2, value.3, value.4);
        Self::new(head, tail)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instructions() {
        let head = Head::new(State::invalid(), "b");
        let tail = Tail::new(State::invalid(), "a", Move::Right);
        let instructions = Instruction::new(head, tail);

        assert_eq!(instructions.tail().action(), Move::Right)
    }
}
