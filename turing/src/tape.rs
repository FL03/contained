/*
    Appellation: tape <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Tape
///
/// The [Tape] represents the memory of a turing machine.
use super::Symbolic;
use contained_core::{ArrayLike, Insert, Iterable};
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Tape<S: Symbolic = String>(Vec<S>);

impl<S: Symbolic> Tape<S> {
    pub fn new() -> Self {
        Self(Default::default())
    }
    /// Creates a new tape from an iterator; preserves the original order.
    pub fn norm(iter: impl IntoIterator<Item = S>) -> Self {
        Self::from_iter(iter)
    }
    /// Creates a new tape from an iterator; the tape is reversed.
    pub fn std(iter: impl IntoIterator<Item = S>) -> Self {
        let mut tape = Vec::from_iter(iter);
        tape.reverse();
        Self::from_iter(tape.clone())
    }
    /// Creates a new tape with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
    pub fn write(&mut self, index: usize, symbol: S) {
        self.0.insert(index, symbol)
    }
}

impl<S: Symbolic> ArrayLike<S> for Tape<S> {}

impl<S: Symbolic> AsMut<Vec<S>> for Tape<S> {
    fn as_mut(&mut self) -> &mut Vec<S> {
        self.0.as_mut()
    }
}

impl<S: Symbolic> AsRef<Vec<S>> for Tape<S> {
    fn as_ref(&self) -> &Vec<S> {
        self.0.as_ref()
    }
}

impl<S: Symbolic> Extend<S> for Tape<S> {
    fn extend<T: IntoIterator<Item = S>>(&mut self, iter: T) {
        self.as_mut().extend(iter);
    }
}

impl<S: Symbolic> FromIterator<S> for Tape<S> {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl<S: Symbolic> Index<usize> for Tape<S> {
    type Output = S;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<S: Symbolic> IndexMut<usize> for Tape<S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<S: Symbolic> Insert<usize, S> for Tape<S> {
    fn insert(&mut self, index: usize, elem: S) {
        self.as_mut().insert(index, elem);
    }
}

impl<S: Symbolic> Iterable<usize, S> for Tape<S> {}

impl<S: Symbolic> IntoIterator for Tape<S> {
    type Item = S;
    type IntoIter = std::vec::IntoIter<S>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<S: Symbolic> From<Tape<S>> for Vec<S> {
    fn from(tape: Tape<S>) -> Vec<S> {
        tape.as_ref().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tape() {
        let mut tape = Tape::new();
        assert!(tape.is_empty());
        tape.push("a");
        assert_eq!(tape.len(), 1);
        assert_eq!(tape[0], "a");
        tape.append(&mut Tape::from_iter(vec!["b", "c"]));
        assert_eq!(tape.len(), 3);
        assert_eq!(tape[2], "c");
    }
}
