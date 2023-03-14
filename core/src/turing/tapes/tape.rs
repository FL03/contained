/*
    Appellation: tape <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: The tape structure modifies traditional vectors, restricing the ability to remove entries from the tape.
*/
use crate::Symbolic;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Tape<S: Symbolic = String>(Vec<S>);

impl<S: Symbolic> Tape<S> {
    pub fn new(symbols: impl IntoIterator<Item = S>) -> Self {
        Self(Vec::from_iter(symbols))
    }
    pub fn norm(iter: impl IntoIterator<Item = S>) -> Self {
        Self::new(iter)
    }
    pub fn std(iter: impl IntoIterator<Item = S>) -> Self {
        let mut tape = Vec::from_iter(iter);
        tape.reverse();
        Self::new(tape)
    }
    pub fn get(&self, pos: usize) -> Option<&S> {
        self.tape().get(pos)
    }
    pub fn insert(&mut self, pos: usize, elem: S) {
        self.tape_mut().insert(pos, elem);
    }
    pub fn is_empty(&self) -> bool {
        self.tape().is_empty()
    }
    pub fn len(&self) -> usize {
        self.tape().len()
    }
    pub fn push(&mut self, elem: S) {
        self.tape_mut().push(elem);
    }
    pub fn set(&mut self, index: usize, elem: S) {
        self.0[index] = elem;
    }
    pub fn tape(&self) -> &Vec<S> {
        &self.0
    }
    pub fn tape_mut(&mut self) -> &mut Vec<S> {
        &mut self.0
    }
}

impl<S: Symbolic> IntoIterator for Tape<S> {
    type Item = S;
    type IntoIter = std::vec::IntoIter<S>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
