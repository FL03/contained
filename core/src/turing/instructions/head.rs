/*
    Appellation: head <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        The instruction head is a two-tuple (State, Symbol)
*/
use crate::{
    states::{State, Stateful},
    turing::{Operator, Symbolic},
    Scope,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Head<S: Symbolic>(State, S);

impl<S: Symbolic> Head<S> {
    pub fn new(state: State, symbol: S) -> Self {
        Self(state, symbol)
    }
    pub fn symbol(&self) -> S {
        self.1.clone()
    }
}

impl<S: Symbolic> std::fmt::Display for Head<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl<S: Symbolic> Stateful<State> for Head<S> {
    fn state(&self) -> State {
        self.0
    }

    fn update_state(&mut self, state: State) {
        self.0 = state;
    }
}

impl<S: Symbolic> From<Operator<S>> for Head<S> {
    fn from(value: Operator<S>) -> Self {
        Self::new(value.state(), value.scope().clone())
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
