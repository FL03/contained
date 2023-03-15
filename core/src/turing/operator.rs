/*
    Appellation: operator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::tapes::{Tape, Tapes};
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
    pub fn new(index: RefCell<usize>, state: State, tape: Tape<S>) -> Self {
        Self { index, state, tape }
    }
    pub fn build(tape: Tapes<S>) -> Self {
        match tape {
            Tapes::Normal(t) => Self::new(0.into(), Default::default(), t),
            Tapes::Standard(t) => Self::new((t.len() - 1).into(), Default::default(), t),
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
        let i = self.index();
        self.index.replace(i + 1);
        if let Some(cur) = self.tape.get(i) {
            Some(cur.clone())
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

impl<S: Symbolic> Stateful<State> for Operator<S> {
    fn state(&self) -> State {
        self.state
    }

    fn update_state(&mut self, state: State) {
        self.state = state;
    }
}

impl<S: Ord + Symbolic> std::fmt::Display for Operator<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {:?}", self.index(), self.state, self.tape)
    }
}

impl<S: Symbolic> TryFrom<(usize, State, Tape<S>)> for Operator<S> {
    type Error = String;
    fn try_from(d: (usize, State, Tape<S>)) -> Result<Self, Self::Error> {
        if d.0 > d.2.len() {
            return Err("Starting index is out of bounds...".to_string());
        }
        Ok(Self::new(d.0.into(), d.1, d.2))
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
    use crate::turing::{instructions::Move, tapes::Tapes};

    #[test]
    fn test_builder() {
        let tape = Tape::new(["a", "b", "c"]);
        let a = Operator::build(Tapes::Normal(tape.clone()));
        let b = Operator::build(Tapes::Standard(tape));
        assert_ne!(a, b);
    }

    #[test]
    fn test_operations() {
        let tape = Tape::new(["a", "b", "c"]);
        let mut actor = Operator::new(0.into(), State::from(State::Valid), tape);

        actor.shift(Move::Left, "b");
        assert_eq!(actor.tape(), &Tape::new(["b", "a", "b", "c"]));
        for _ in 0..actor.tape().len() {
            actor.shift(Move::Right, "b");
        }
        assert_eq!(actor.tape(), &Tape::new(["b", "a", "b", "c", "b"]));
    }
}
