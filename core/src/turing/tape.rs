/*
    Appellation: tape <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: The tape structure modifies traditional vectors, restricing the ability to remove entries from the tape.
*/
use crate::Symbolic;
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
        Self(Vec::from_iter(iter))
    }
    /// Creates a new tape from an iterator; the tape is reversed.
    pub fn std(iter: impl IntoIterator<Item = S>) -> Self {
        let mut tape = Vec::from_iter(iter);
        tape.reverse();
        Self(tape.clone())
    }
    /// Returns the element at the given index, or `None` if the index is out of bounds.
    pub fn get(&self, index: usize) -> Option<&S> {
        self.as_ref().get(index)
    }
    /// Inserts an element at the given index, shifting all elements after it to the right.
    pub fn insert(&mut self, index: usize, elem: S) {
        self.as_mut().insert(index, elem);
    }
    /// Returns `true` if the tape contains no elements.
    pub fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }
    /// Returns the number of elements in the tape.
    pub fn len(&self) -> usize {
        self.as_ref().len()
    }
    /// Pushes an element to the end of the tape.
    pub fn push(&mut self, elem: S) {
        self.as_mut().push(elem);
    }
    /// Sets the element at the given index
    pub fn set(&mut self, index: usize, elem: S) {
        self[index] = elem;
    }
    /// Returns a reference to the underlying vector
    pub fn tape(&self) -> &Vec<S> {
        &self.0
    }
    /// Creates a new tape with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl<S: Symbolic> AsMut<Vec<S>> for Tape<S> {
    fn as_mut(&mut self) -> &mut Vec<S> {
        &mut self.0
    }
}

impl<S: Symbolic> AsRef<Vec<S>> for Tape<S> {
    fn as_ref(&self) -> &Vec<S> {
        self.tape()
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

impl<S: Symbolic> IntoIterator for Tape<S> {
    type Item = S;
    type IntoIter = std::vec::IntoIter<S>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<S: Symbolic> From<&[S]> for Tape<S> {
    fn from(d: &[S]) -> Tape<S> {
        Tape(d.into_iter().cloned().collect())
    }
}

impl<S: Symbolic> From<Vec<S>> for Tape<S> {
    fn from(d: Vec<S>) -> Tape<S> {
        Tape(d)
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

        let a = Tape::from(alpha);
        assert_eq!(a.len(), 3);
    }
}
