/*
    Appellation: tail <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Turing machines accept instructions in the form of a five-tuple:
            (State, Symbol, State, Symbol, Move)
*/
use super::Move;
use crate::{
    states::{State, States},
    turing::Symbolic,
};
use serde::{Deserialize, Serialize};

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
