/*
    Appellation: instructions <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Turing machines accept instructions in the form of a five-tuple:
            (State, Symbol, State, Symbol, Move)
*/
use crate::{turing::Symbolic, State, States};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Head<S: Symbolic>(State<States>, S);

impl<S: Symbolic> Head<S> {
    pub fn new(state: State<States>, symbol: S) -> Self {
        Self(state, symbol)
    }
    pub fn state(&self) -> &State<States> {
        &self.0
    }
    pub fn symbol(&self) -> &S {
        &self.1
    }
}

impl<S: Symbolic> From<Head<S>> for (State<States>, S) {
    fn from(v: Head<S>) -> (State<States>, S) {
        (v.0, v.1)
    }
}

impl<S: Symbolic> From<(State<States>, S)> for Head<S> {
    fn from(value: (State<States>, S)) -> Self {
        Self(value.0, value.1)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Tail<S: Symbolic>(State<States>, S, Move);

impl<S: Symbolic> Tail<S> {
    pub fn new(state: State<States>, symbol: S, act: Move) -> Self {
        Self(state, symbol, act)
    }
    pub fn action(&self) -> &Move {
        &self.2
    }
    pub fn state(&self) -> &State<States> {
        &self.0
    }
    pub fn symbol(&self) -> &S {
        &self.1
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Instruction<S: Symbolic>(Head<S>, Tail<S>);

impl<S: Symbolic> Instruction<S> {
    pub fn new(head: Head<S>, tail: Tail<S>) -> Self {
        Self(head, tail)
    }
    pub fn head(&self) -> &Head<S> {
        &self.0
    }
    pub fn tail(&self) -> &Tail<S> {
        &self.1
    }
    pub fn update(&mut self, head: Head<S>, tail: Tail<S>) {
        self.0 = head;
        self.1 = tail;
    }
}

impl<S: Symbolic> From<(State<States>, S, State<States>, S, Move)> for Instruction<S> {
    fn from(value: (State<States>, S, State<States>, S, Move)) -> Self {
        let head = Head::new(value.0, value.1);
        let tail = Tail::new(value.2, value.3, value.4);
        Self::new(head, tail)
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum Move {
    Left = 0,
    Right = 1,
    #[default]
    Stay = 2,
}

impl From<i64> for Move {
    fn from(d: i64) -> Self {
        match (d % 3).abs() {
            0 => Self::Left,
            1 => Self::Right,
            _ => Self::Stay,
        }
    }
}

impl From<Move> for i64 {
    fn from(d: Move) -> i64 {
        d as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::States;

    #[test]
    fn test_instructions() {
        let head = Head::new(State::new(States::invalid()), "b");
        let tail = Tail::new(State::new(States::invalid()), "a", Move::Right);
        let instructions = Instruction::new(head, tail);
        assert_eq!(instructions.tail().action(), &Move::Right)
    }

    #[test]
    fn test_move_default() {
        let a = Move::default();
        assert_eq!(a.clone(), Move::Stay);
    }
}
