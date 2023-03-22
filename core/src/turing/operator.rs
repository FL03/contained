/*
    Appellation: operator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::Tape;
use crate::states::{State, Stateful};
use crate::{Scope, Symbolic};

use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Operator<S: Symbolic = String> {
    index: RefCell<usize>,
    state: State,
    tape: Tape<S>,
}

impl<S: Symbolic> Operator<S> {
    pub fn new(state: State, tape: Tape<S>) -> Self {
        Self {
            index: 0.into(),
            state,
            tape,
        }
    }
}

impl<S: Symbolic> ExactSizeIterator for Operator<S> {
    fn len(&self) -> usize {
        self.tape.len()
    }
}

impl<S: Symbolic> Iterator for Operator<S> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.tape.get(self.index()).cloned() {
            self.index.replace(self.index() + 1);
            Some(cur)
        } else {
            None
        }
    }
}

impl<S: Symbolic> Scope<S> for Operator<S> {
    fn insert(&mut self, elem: S) {
        self.tape.insert(self.index(), elem);
    }

    fn index(&self) -> usize {
        *self.index.borrow()
    }

    fn set_symbol(&mut self, elem: S) {
        self.tape.set(self.index(), elem);
    }

    fn tape(&self) -> &Tape<S> {
        &self.tape
    }

    fn set_index(&mut self, pos: usize) {
        self.index.replace(pos);
    }
}

impl<S: Symbolic> Stateful for Operator<S> {
    type State = State;

    fn state(&self) -> Self::State {
        self.state
    }

    fn update_state(&mut self, state: Self::State) {
        self.state = state;
    }
}

impl<S: Ord + Symbolic> std::fmt::Display for Operator<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {:?}", self.index(), self.state, self.tape)
    }
}

impl<S: Symbolic> From<Tape<S>> for Operator<S> {
    fn from(tape: Tape<S>) -> Self {
        Self::new(State::Valid, tape)
    }
}

impl<S: Symbolic> TryFrom<(usize, State, Tape<S>)> for Operator<S> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(d: (usize, State, Tape<S>)) -> Result<Self, Self::Error> {
        if d.0 > d.2.len() {
            return Err("Starting index is out of bounds...".into());
        }
        Ok(Self {
            index: d.0.into(),
            state: d.1,
            tape: d.2,
        })
    }
}

impl<S: Symbolic> From<Operator<S>> for (usize, State, Tape<S>) {
    fn from(d: Operator<S>) -> Self {
        (d.index(), d.state, d.tape)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::turing::{instructions::Move, Tape};

    #[test]
    fn test_builder() {
        let tape = ["a", "b", "c"];
        assert_ne!(Tape::norm(tape.clone()), Tape::std(tape));
    }

    #[test]
    fn test_operations() {
        let tape = Tape::from_iter(["a", "b", "c"]);
        let mut actor = Operator::new(State::Valid, tape);

        actor.shift(Move::Left, "b");
        assert_eq!(actor.tape(), &Tape::from_iter(["b", "a", "b", "c"]));
        for _ in 0..actor.tape().len() {
            actor.shift(Move::Right, "b");
        }
        assert_eq!(actor.tape(), &Tape::from_iter(["b", "a", "b", "c", "b"]));
    }
}
