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
use crate::turing::{Configuration, Symbolic, Tape};
use serde::{Deserialize, Serialize};

pub trait Triadic {
    fn is_valid(&self) -> bool {
        if self.root() != self.third()
            && self.root() != self.fifth()
            && self.third() != self.fifth()
        {
            return true;
        }
        false
    }
    fn chord(&self) -> (Note, Note, Note);
    fn root(&self) -> Note {
        self.chord().0
    }
    fn third(&self) -> Note {
        self.chord().1
    }
    fn fifth(&self) -> Note {
        self.chord().2
    }
    fn update(&mut self, root: Option<Note>, third: Option<Note>, fifth: Option<Note>);
}

/// [Triad] is a set of three [Note], the root, third, and fifth.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Triad(Note, Note, Note);

impl Triad {
    pub fn new(root: Note, third: Note, fifth: Note) -> Self {
        Self(root, third, fifth)
    }
}

impl Triadic for Triad {
    fn chord(&self) -> (Note, Note, Note) {
        self.clone().into()
    }

    fn update(&mut self, root: Option<Note>, third: Option<Note>, fifth: Option<Note>) {
        if let Some(n) = root {
            self.0 = n;
        }
        if let Some(n) = third {
            self.1 = n;
        }
        if let Some(n) = fifth {
            self.2 = n;
        }
    }
}

impl std::fmt::Display for Triad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0, self.1, self.2)
    }
}

impl Symbolic for Triad {}

impl From<Triad> for Configuration<Note> {
    fn from(value: Triad) -> Configuration<Note> {
        let triad: Tape<Note> = value.into();
        Configuration::norm(triad).unwrap()
    }
}

impl From<Triad> for Vec<Note> {
    fn from(value: Triad) -> Vec<Note> {
        vec![value.0, value.1, value.2]
    }
}

impl From<Triad> for Tape<Note> {
    fn from(value: Triad) -> Tape<Note> {
        let t = [value.0, value.1, value.2];
        Tape::new(t)
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

impl From<(Note, Note, Note)> for Triad {
    fn from(d: (Note, Note, Note)) -> Triad {
        Triad(d.0, d.1, d.2)
    }
}

impl From<Triad> for (Note, Note, Note) {
    fn from(d: Triad) -> (Note, Note, Note) {
        (d.0, d.1, d.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triad() {
        let a = Triad::from((0, 1, 2));
        let b = Triad::from((2, 1, 0));
        assert!(a.is_valid());
        assert!(b.is_valid());
        assert_ne!(a, b)
    }
}
