/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A triad is a set of three notes, called chord factors: root, third, and fifth
        Generaically, triad's share two of its notes with three of its inversions.

        We express a triad as a ordered three tuple <a, b, c> where a, b, c are integers modulus of 12 and:
            a != b
            a != c
            b != c
*/
use crate::neo::{cmp::Note, LPR};
use crate::turing::{Configuration, Symbolic, Tape, Turing};
use serde::{Deserialize, Serialize};

pub trait Triadic {
    fn fifth(&self) -> Note;
    fn root(&self) -> Note;
    fn third(&self) -> Note;
    fn triad(&self) -> Triad;
}

pub enum Triads {
    Augmented(Triad),
    Major(Triad),
    Minor(Triad),
}

/// [Triad] is a set of three [Note], the root, third, and fifth.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Triad(Note, Note, Note);

impl Triad {
    pub fn new(root: Note, third: Note, fifth: Note) -> Self {
        Self(root, third, fifth)
    }
    /// Create a new [Configuration] with the [Triad] as its alphabet
    pub fn config(&self) -> Configuration<Note> {
        Configuration::norm(Tape::new(self.clone())).unwrap()
    }
    pub fn is_valid(&self) -> bool {
        if self.root() < self.third() && self.root() < self.fifth() && self.third() < self.fifth() {
            return true;
        }
        false
    }
    pub fn fifth(&self) -> &Note {
        &self.2
    }
    pub fn root(&self) -> &Note {
        &self.0
    }
    pub fn third(&self) -> &Note {
        &self.1
    }
}

impl Symbolic for Triad {}

impl std::fmt::Display for Triad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0, self.1, self.2)
    }
}

impl std::ops::Mul<LPR> for Triad {
    type Output = Triad;

    fn mul(self, rhs: LPR) -> Self::Output {
        rhs.transform(&mut self.clone())
    }
}

impl IntoIterator for Triad {
    type Item = Note;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.0, self.1, self.2].into_iter()
    }
}

impl From<Triad> for (i64, i64, i64) {
    fn from(d: Triad) -> (i64, i64, i64) {
        (
            d.0.pitch().clone().into(),
            d.1.pitch().clone().into(),
            d.2.pitch().clone().into(),
        )
    }
}

impl From<(i64, i64, i64)> for Triad {
    fn from(d: (i64, i64, i64)) -> Triad {
        Triad(d.0.into(), d.1.into(), d.2.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triad() {
        let a = Triad::from((0, 1, 2));
        let b = Triad::from((2, 4, 9));
        assert!(a.is_valid());
        assert!(b.is_valid());
        assert_ne!(a, b)
    }
}
