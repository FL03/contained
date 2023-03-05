/*
    Appellation: tapes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: The tape structure modifies traditional vectors, restricing the ability to remove entries from the tape.
*/
use crate::Symbolic;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]

pub enum Tapes<S: Symbolic = String> {
    Normal(Tape<S>),
    Standard(Tape<S>),
}

impl<S: Symbolic> Tapes<S> {
    pub fn normal(tape: Tape<S>) -> Self {
        Self::Normal(tape)
    }
    pub fn standard(tape: Tape<S>) -> Self {
        Self::Standard(tape)
    }
}

impl<S: Symbolic> Default for Tapes<S> {
    fn default() -> Self {
        Self::normal(Default::default())
    }
}

impl<S: Symbolic> From<Tapes<S>> for Tape<S> {
    fn from(tape: Tapes<S>) -> Tape<S> {
        match tape {
            Tapes::Normal(t) => t,
            Tapes::Standard(t) => t,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Tape<S: Symbolic = String>(Vec<S>);

impl<S: Symbolic> Tape<S> {
    pub fn new(symbols: impl IntoIterator<Item = S>) -> Self {
        Self(Vec::from_iter(symbols))
    }
    pub fn get(&self, pos: usize) -> Option<&S> {
        self.tape().get(pos)
    }
    pub fn insert(&mut self, pos: usize, elem: S) {
        self.0.insert(pos, elem);
    }
    pub fn is_empty(&self) -> bool {
        self.tape().is_empty()
    }
    pub fn len(&self) -> usize {
        self.tape().len()
    }
    pub fn push(&mut self, elem: S) {
        self.0.push(elem);
    }
    pub fn set(&mut self, index: usize, elem: S) {
        self.0[index] = elem;
    }
    pub fn tape(&self) -> &Vec<S> {
        &self.0
    }
}

impl<S: Symbolic> From<Vec<S>> for Tape<S> {
    fn from(d: Vec<S>) -> Tape<S> {
        Tape::new(d)
    }
}

impl<S: Symbolic> From<Tape<S>> for Vec<S> {
    fn from(d: Tape<S>) -> Vec<S> {
        d.tape().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tape() {
        let alpha = vec!["a", "b", "c"];

        let a = Tape::new(alpha);
        assert_eq!(a.len(), 3);
    }
}
