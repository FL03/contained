/*
    Appellation: tail <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Turing machines accept instructions in the form of a five-tuple:
            (State, Symbol, State, Symbol, Move)
*/
use super::Move;
use crate::{
    states::{State, Stateful},
    Symbolic,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Tail<S: Symbolic>(State, S, Move);

impl<S: Symbolic> Tail<S> {
    pub fn new(state: State, symbol: S, act: Move) -> Self {
        Self(state, symbol, act)
    }
    pub fn action(&self) -> Move {
        self.2
    }
    pub fn state(&self) -> State {
        self.0.clone()
    }
    pub fn symbol(&self) -> S {
        self.1.clone()
    }
}

impl<S: Symbolic> std::fmt::Display for Tail<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl<S: Symbolic> Stateful<State> for Tail<S> {
    fn state(&self) -> State {
        self.0
    }

    fn update_state(&mut self, state: State) {
        self.0 = state;
    }
}
