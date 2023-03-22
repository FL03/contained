/*
    Appellation: head <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        The instruction head is a two-tuple (State, Symbol)
*/
use crate::{
    states::{State, Stateful},
    turing::{Driver, Symbolic},
    Scope,
};
use decanter::prelude::{hasher, Hashable, H256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Head<S: Symbolic> {
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

impl<S: Symbolic> Hashable for Head<S> {
    fn hash(&self) -> H256 {
        hasher(self).into()
    }
}

impl<S: Symbolic> Stateful for Head<S> {
    type State = State;

    fn state(&self) -> Self::State {
        self.state
    }

    fn update_state(&mut self, state: Self::State) {
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
        Self::new(value.state(), value.symbol().clone())
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
