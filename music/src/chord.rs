/*
    Appellation: chord <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A chord is any set of notes played simultaneously; for our considerations, allow a chord to represent the alphabet of a Turing machine or automata.
*/
use super::Note;
use contained_core::{ArrayLike, Insert, Iterable};
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

/// [Chord] is a wrapper for a [Vec] of [Note]
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Chord {
    chord: Vec<Note>,
}

impl Chord {
    pub fn new() -> Self {
        Self { chord: Vec::new() }
    }
    pub fn is_triadic(&self) -> bool {
        self.len() == 3
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            chord: Vec::with_capacity(capacity),
        }
    }
}

impl ArrayLike<Note> for Chord {}

impl AsMut<Vec<Note>> for Chord {
    fn as_mut(&mut self) -> &mut Vec<Note> {
        &mut self.chord
    }
}

impl AsRef<Vec<Note>> for Chord {
    fn as_ref(&self) -> &Vec<Note> {
        &self.chord
    }
}

impl Extend<Note> for Chord {
    fn extend<T: IntoIterator<Item = Note>>(&mut self, iter: T) {
        self.chord.extend(iter)
    }
}

impl FromIterator<Note> for Chord {
    fn from_iter<T: IntoIterator<Item = Note>>(iter: T) -> Self {
        Self {
            chord: Vec::from_iter(iter),
        }
    }
}

impl Index<usize> for Chord {
    type Output = Note;

    fn index(&self, index: usize) -> &Self::Output {
        &self.chord[index]
    }
}

impl IndexMut<usize> for Chord {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.chord[index]
    }
}

impl Insert<usize, Note> for Chord {
    fn insert(&mut self, index: usize, elem: Note) {
        self.as_mut().insert(index, elem);
    }
}

impl Iterable<usize, Note> for Chord {}

impl IntoIterator for Chord {
    type Item = Note;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.chord.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chord() {
        // Create a new chord
        let mut chord = Chord::new();
        // Assert that the chord is empty
        assert!(chord.is_empty());
        // Append a chord to the chord
        chord.append(&mut Chord::from_iter([0.into(), 3.into(), 8.into()]));
        // Assert that the chord is not empty; it should have 3 notes
        assert_eq!(chord.len(), 3);
        // Assert that the first note is 0
        assert_eq!(chord[0], 0.into());
        // Set the first note to 1 via the index operator
        chord[0] = 1.into();
        // Assert that the first note is now 1
        assert_eq!(chord[0], 1.into());
        assert_eq!(chord.as_ref(), &[1.into(), 3.into(), 8.into()]);
    }

    #[test]
    fn test_chord_from_iter() {
        let chord = Chord::from_iter([0.into(), 3.into(), 8.into()]);
        assert_eq!(chord[0], 0.into());
        assert_eq!(chord[1], 3.into());
        assert_eq!(chord[2], 8.into());
    }
}
