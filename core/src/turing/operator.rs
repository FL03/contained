/*
    Appellation: operator <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::states::{State, States};
use crate::turing::Tape;
use crate::{Scope, Symbolic};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Operator<S: Symbolic = String> {
    index: usize,
    pub(crate) state: State<States>,
    tape: Tape<S>,
}

impl<S: Symbolic> Scope<S> for Operator<S> {
    fn new(index: usize, state: State<States>, tape: Tape<S>) -> Self {
        Self { index, state, tape }
    }

    fn insert(&mut self, elem: S) {
        self.tape.insert(self.position(), elem);
    }

    fn position(&self) -> usize {
        self.index
    }

    fn set_position(&mut self, index: usize) {
        self.index = index;
    }
    fn set_state(&mut self, state: State<States>) {
        self.state = state;
    }
    fn set_symbol(&mut self, elem: S) {
        self.tape.set(self.position(), elem);
    }

    fn state(&self) -> &State<States> {
        &self.state
    }

    fn tape(&self) -> &Tape<S> {
        &self.tape
    }
}

impl<S: Ord + Symbolic> std::fmt::Display for Operator<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {:?}", self.index, self.state, self.tape)
    }
}

impl<S: Symbolic> TryFrom<(usize, State<States>, Tape<S>)> for Operator<S> {
    type Error = String;
    fn try_from(d: (usize, State<States>, Tape<S>)) -> Result<Self, Self::Error> {
        if d.0 > d.2.len() {
            return Err("Starting index is out of bounds...".to_string());
        }
        Ok(Self::new(d.0, d.1, d.2))
    }
}

impl<S: Symbolic> From<Operator<S>> for (usize, State<States>, Tape<S>) {
    fn from(d: Operator<S>) -> Self {
        (d.index, d.state, d.tape)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::turing::{Move, Tapes};

    #[test]
    fn test_builder() {
        let tape = Tape::new(["a", "b", "c"]);
        let a = Operator::build(Tapes::normal(tape.clone()));
        let b = Operator::build(Tapes::standard(tape));
        assert_ne!(a, b);
    }

    #[test]
    fn test_operations() {
        let tape = Tape::new(["a", "b", "c"]);
        let mut actor = Operator::new(0, State::from(States::Valid), tape);
        actor.shift(Move::Left, "b");
        assert_eq!(actor.tape(), &Tape::new(["b", "a", "b", "c"]));
        for _ in 0..actor.tape().len() {
            actor.shift(Move::Right, "b");
        }
        assert_eq!(actor.tape(), &Tape::new(["b", "a", "b", "c", "b"]));
    }
}
