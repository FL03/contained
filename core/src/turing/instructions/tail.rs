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
use decanter::prelude::{hasher, Hashable, H256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Tail<S: Symbolic> {
    state: State,
    symbol: S,
    action: Move,
}

impl<S: Symbolic> Tail<S> {
    pub fn new(state: State, symbol: S, action: Move) -> Self {
        Self {
            state,
            symbol,
            action,
        }
    }
    pub fn action(&self) -> Move {
        self.action
    }
    pub fn state(&self) -> State {
        self.state
    }
    pub fn symbol(&self) -> S {
        self.symbol.clone()
    }
}

impl<S: Symbolic> Hashable for Tail<S> {
    fn hash(&self) -> H256 {
        hasher(self).into()
    }
}

impl<S: Symbolic> Stateful<State> for Tail<S> {
    fn state(&self) -> State {
        self.state
    }

    fn update_state(&mut self, state: State) {
        self.state = state;
    }
}

impl<S: Symbolic> std::fmt::Display for Tail<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.state, self.symbol, self.action)
    }
}

impl<S: Symbolic> From<(State, S, Move)> for Tail<S> {
    fn from(args: (State, S, Move)) -> Self {
        Self::new(args.0, args.1, args.2)
    }
}

impl<S: Symbolic> From<Tail<S>> for (State, S, Move) {
    fn from(tail: Tail<S>) -> (State, S, Move) {
        (tail.state(), tail.symbol(), tail.action())
    }
}
