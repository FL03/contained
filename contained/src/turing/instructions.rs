/*
    Appellation: reactions <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Turing machines are given three degrees of freedom when considering possible movements

*/
use crate::{turing::Symbolic, State};
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Head<T: Symbolic>(State, T);

impl<T: Symbolic> Head<T> {
    pub fn new(state: State, symbol: T) -> Self {
        Self(state, symbol)
    }
    pub fn state(&self) -> &State {
        &self.0
    }
    pub fn symbol(&self) -> &T {
        &self.1
    }
}

impl<S: Symbolic> From<Head<S>> for (State, S) {
    fn from(v: Head<S>) -> (State, S) {
        (v.0, v.1)
    }
}

impl<S: Symbolic> From<(State, S)> for Head<S> {
    fn from(value: (State, S)) -> Self {
        Self(value.0, value.1)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Tail<S: Symbolic>(State, S, Move);

impl<S: Symbolic> Tail<S> {
    pub fn new(state: State, symbol: S, act: Move) -> Self {
        Self(state, symbol, act)
    }
    pub fn action(&self) -> &Move {
        &self.2
    }
    pub fn state(&self) -> &State {
        &self.0
    }
    pub fn symbol(&self) -> &S {
        &self.1
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Instruction<S: Symbolic> {
    pub head: Head<S>,
    pub tail: Tail<S>,
}

impl<S: Symbolic> Instruction<S> {
    pub fn new(head: Head<S>, tail: Tail<S>) -> Self {
        Self { head, tail }
    }
}

impl<S: Symbolic> From<(State, S, State, S, Move)> for Instruction<S> {
    fn from(value: (State, S, State, S, Move)) -> Self {
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
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
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

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = match *self as i64 {
            0 => "left",
            1 => "right",
            _ => "stay",
        };
        write!(f, "{}", p)
    }
}

impl From<i64> for Move {
    fn from(d: i64) -> Self {
        match d {
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
        let head = Head::new(State::new(None, States::invalid()), "b");
        let tail = Tail::new(State::new(None, States::invalid()), "a", Move::Right);
        let instructions = Instruction::new(head, tail);
        assert_eq!(instructions.tail.action(), &Move::Right)
    }

    #[test]
    fn test_move_default() {
        let a = Move::default();
        assert_eq!(a.clone(), Move::Stay);
    }
}
