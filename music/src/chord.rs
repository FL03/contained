/*
    Appellation: chord <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A chord is any set of notes played simultaneously; for our considerations, allow a chord to represent the alphabet of a Turing machine or automata.

*/
use super::Note;
use contained_core::ArrayLike;
use serde::{Deserialize, Serialize};

/// [Chord] is a wrapper for a [Vec] of [Note]
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Chord(Vec<Note>);

impl Chord {
    pub fn new(chord: impl IntoIterator<Item = Note>) -> Self {
        Self(Vec::from_iter(chord))
    }
    pub fn chord(&self) -> &Self {
        self
    }
    pub fn is_triadic(&self) -> bool {
        self.len() == 3
    }
}

impl ArrayLike<Note> for Chord {
    fn content(&self) -> &Vec<Note> {
        &self.0
    }

    fn mut_content(&mut self) -> &mut Vec<Note> {
        &mut self.0
    }
}

impl FromIterator<Note> for Chord {
    fn from_iter<T: IntoIterator<Item = Note>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl IntoIterator for Chord {
    type Item = Note;

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
        let mut a: Vec<Note> = vec![0.into(), 3.into(), 8.into()];
        let mut b = Chord::default();
        assert!(b.is_empty());
        b.append(&mut a);
        assert_eq!(b.len(), 3);
    }
}
