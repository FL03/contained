/*
    Appellation: chord <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A chord is any set of notes played simultaneously; for our considerations, allow a chord to represent the alphabet of a Turing machine or automata.

*/
use crate::{
    music::{Notable, Note},
    ArrayLike,
};
use serde::{Deserialize, Serialize};

/// [Chord] is a wrapper for a [Vec] of [Note]
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Chord<N: Notable = Note>(Vec<N>);

impl<N: Notable> Chord<N> {
    pub fn new(chord: impl IntoIterator<Item = N>) -> Self {
        Self(Vec::from_iter(chord))
    }
    pub fn chord(&self) -> &Self {
        self
    }
    pub fn is_triadic(&self) -> bool {
        self.len() == 3
    }
}

impl<N: Notable> ArrayLike for Chord<N> {
    type Data = N;

    fn content(&self) -> &Vec<Self::Data> {
        &self.0
    }

    fn mut_content(&mut self) -> &mut Vec<Self::Data> {
        &mut self.0
    }
}

impl<N: Notable> IntoIterator for Chord<N> {
    type Item = N;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chords() {
        let a: Vec<Note> = vec![0.into(), 3.into(), 8.into()];
        let mut b = Chord::default();
        assert!(b.is_empty());
        b.append(&mut a.clone());
        assert_eq!(b.len(), 3);
    }
}
