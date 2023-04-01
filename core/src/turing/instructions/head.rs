/*
    Appellation: head <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        The instruction head is a two-tuple (State, Symbol)
*/
use crate::{
    turing::{Driver, Symbolic},
    Scope, State, Stateful,
};
use decanter::prelude::Hashable;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Default, Deserialize, Eq, Hash, Hashable, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Head<S: Symbolic = String> {
    state: State,
    symbol: S,
}

impl<S: Symbolic> Head<S> {
    pub fn new(state: State, symbol: S) -> Self {
        Self { state, symbol }
    }
    pub fn symbol(&self) -> S {
        self.symbol.clone()
    }
}

impl<S: Symbolic> Stateful<State> for Head<S> {
    fn state(&self) -> State {
        self.state
    }

    fn update_state(&mut self, state: State) {
        self.state = state;
    }
}

impl<S: Symbolic> std::fmt::Display for Head<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.state, self.symbol)
    }
}

impl<S: Symbolic> From<Driver<S>> for Head<S> {
    fn from(value: Driver<S>) -> Self {
        Self::new(value.state(), value.current())
    }
}

impl<S: Symbolic> From<Head<S>> for (State, S) {
    fn from(v: Head<S>) -> (State, S) {
        (v.state(), v.symbol())
    }
}

impl<S: Symbolic> From<(State, S)> for Head<S> {
    fn from(value: (State, S)) -> Self {
        Self::new(value.0, value.1)
    }
}
