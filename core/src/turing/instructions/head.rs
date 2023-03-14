/*
    Appellation: head <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        The instruction head is a two-tuple (State, Symbol)
*/
use crate::{
    states::{State, Stateful, States},
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
    pub fn state(&self) -> State {
        self.0.clone()
    }
    pub fn symbol(&self) -> S {
        self.1.clone()
    }
}

impl<S: Symbolic> From<Operator<S>> for Head<S> {
    fn from(value: Operator<S>) -> Self {
        Self::new(value.state().clone().state().into(), value.scope().clone())
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
