/*
    Appellation: reactions <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{State, Symbolic};
use scsys::prelude::fnl_remove;
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct Head<S: Symbolic>(State, S);

impl<S: Symbolic> Head<S> {
    pub fn new(state: State, symbol: S) -> Self {
        Self(state, symbol)
    }
    pub fn state(&self) -> &State {
        &self.0
    }
    pub fn symbol(&self) -> &S {
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

pub struct Instruction<S: Symbolic> {
    pub head: Head<S>,
    pub tail: Tail<S>
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

#[derive(Clone, Copy, Debug, Default, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, PartialOrd, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum Move {
    Left = 0,
    Right = 1,
    #[default]
    Stay = 2
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", fnl_remove(serde_json::to_string(self).unwrap()))
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


    #[test]
    fn test_move_default() {
        let a = Move::default();
        assert_eq!(a.clone(), Move::Stay);
    }
}