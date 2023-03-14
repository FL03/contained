/*
    Appellation: operator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::tapes::Tape;
use crate::states::{State, States};
use crate::{Scope, Symbolic};

use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Operator<S: Symbolic = String> {
    index: RefCell<usize>,
    state: State<States>,
    tape: Tape<S>,
}

impl<S: Symbolic> Operator<S> {}

impl<S: Symbolic> Scope<S> for Operator<S> {
    fn new(index: RefCell<usize>, state: State<States>, tape: Tape<S>) -> Self {
        Self { index, state, tape }
    }

    fn insert(&mut self, elem: S) {
        self.tape.insert(*self.index.borrow(), elem);
    }

    fn index(&self) -> &RefCell<usize> {
        &self.index
    }

    fn set_symbol(&mut self, elem: S) {
        self.tape.set(*self.index.borrow(), elem);
    }

    fn state(&self) -> &State<States> {
        &self.state
    }

    fn tape(&self) -> &Tape<S> {
        &self.tape
    }

    fn update(&mut self, state: Option<State<States>>, elem: Option<S>) {
        if let Some(s) = state {
            self.state = s;
        }
        if let Some(t) = elem {
            self.tape.set(*self.index.borrow(), t);
        }
    }
}

impl<S: Symbolic> Iterator for Operator<S> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        let i = *self.index.borrow();
        self.index.replace(i + 1);
        if let Some(cur) = self.tape.get(i) {
            Some(cur.clone())
        } else {
            None
        }
    }
}

impl<S: Symbolic> ExactSizeIterator for Operator<S> {
    fn len(&self) -> usize {
        self.tape.len()
    }
}

impl<S: Ord + Symbolic> std::fmt::Display for Operator<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {:?}",
            *self.index.borrow(),
            self.state,
            self.tape
        )
    }
}

impl<S: Symbolic> TryFrom<(usize, State<States>, Tape<S>)> for Operator<S> {
    type Error = String;
    fn try_from(d: (usize, State<States>, Tape<S>)) -> Result<Self, Self::Error> {
        if d.0 > d.2.len() {
            return Err("Starting index is out of bounds...".to_string());
        }
        Ok(Self::new(RefCell::new(d.0), d.1, d.2))
    }
}

impl<S: Symbolic> From<Operator<S>> for (usize, State<States>, Tape<S>) {
    fn from(d: Operator<S>) -> Self {
        (*d.index.borrow(), d.state, d.tape)
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
        let mut actor = Operator::new(0.into(), State::from(States::Valid), tape);

        actor.shift(Move::Left, "b");
        assert_eq!(actor.tape(), &Tape::new(["b", "a", "b", "c"]));
        for _ in 0..actor.tape().len() {
            actor.shift(Move::Right, "b");
        }
        assert_eq!(actor.tape(), &Tape::new(["b", "a", "b", "c", "b"]));
    }
}
