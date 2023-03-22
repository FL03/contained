/*
    Appellation: driver <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::Tape;
use crate::states::{State, Stateful};
use crate::{Include, InsertAt, Scope, Symbolic};

use serde::{Deserialize, Serialize};
use std::cell::RefCell;

/// [Driver] implements the [Scope] trait and essentially represents the focus of a [super::Turing] machine
/// [Driver] is a [Stateful] [Iterator] that tracks the [State] of the [super::Turing] machine and the current position of the [Tape]
#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Driver<S: Symbolic = String> {
    index: RefCell<usize>,
    state: State,
    pub tape: Tape<S>,
}

impl<S: Symbolic> Driver<S> {
    pub fn new(state: State, tape: Tape<S>) -> Self {
        Self {
            index: 0.into(),
            state,
            tape,
        }
    }
}

impl<S: Symbolic> AsMut<Driver<S>> for Driver<S> {
    fn as_mut(&mut self) -> &mut Driver<S> {
        self
    }
}

impl<S: Symbolic> AsRef<Driver<S>> for Driver<S> {
    fn as_ref(&self) -> &Driver<S> {
        &self
    }
}

impl<S: Symbolic> ExactSizeIterator for Driver<S> {
    fn len(&self) -> usize {
        self.tape.len()
    }
}

impl<S: Symbolic> Include<S> for Driver<S> {
    fn include(&mut self, elem: S) {
        self.tape.insert(self.index(), elem);
    }
}

impl<S: Symbolic> InsertAt<usize, S> for Driver<S> {
    fn insert(&mut self, index: usize, elem: S) {
        self.tape.insert(index, elem);
    }
}

impl<S: Symbolic> Iterator for Driver<S> {
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

impl<S: Symbolic> Scope<S> for Driver<S> {
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

impl<S: Symbolic> Stateful for Driver<S> {
    type State = State;

    fn state(&self) -> Self::State {
        self.state
    }

    fn update_state(&mut self, state: Self::State) {
        self.state = state;
    }
}

impl<S: Ord + Symbolic> std::fmt::Display for Driver<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {:?}", self.index(), self.state, self.tape)
    }
}

impl<S: Symbolic> From<Tape<S>> for Driver<S> {
    fn from(tape: Tape<S>) -> Self {
        Self::new(State::Valid, tape)
    }
}

impl<S: Symbolic> TryFrom<(usize, State, Tape<S>)> for Driver<S> {
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

impl<S: Symbolic> From<Driver<S>> for (usize, State, Tape<S>) {
    fn from(d: Driver<S>) -> Self {
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
        let mut actor = Driver::new(State::Valid, tape);

        actor.shift(Move::Left, "b");
        assert_eq!(actor.tape(), &Tape::from_iter(["b", "a", "b", "c"]));
        for _ in 0..actor.tape().len() {
            actor.shift(Move::Right, "b");
        }
        assert_eq!(actor.tape(), &Tape::from_iter(["b", "a", "b", "c", "b"]));
    }
}
