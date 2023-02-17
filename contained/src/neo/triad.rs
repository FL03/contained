/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A triad is a set of three notes, called chord factors: root, third, and fifth
        Generaically, triad's share two of its notes with three of its inversions.

        The neo-Riemannian theory highlightes three transformations, each of which preserve two of the three notes.

        We express a triad as a ordered three tuple <a, b, c> where a, b, c are integers modulus of 12 and:
            a != b
            a != c
            b != c
*/
use super::LPR;
use crate::cmp::{is_major_third, is_minor_third, is_third, Chord, Fifths, Note, Thirds};
use crate::turing::{Configuration, Machine, Program, Symbolic, Tape};
use crate::Resultant;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

pub trait Triadic: Clone {
    /// [Triadic::chord] Creates a [Chord] from the vertices
    fn chord(&self) -> Chord {
        Chord::new(vec![self.root(), self.third(), self.fifth()])
    }
    /// [Triadic::classify] tries to define the triad by searching for triadic relations
    fn classify(&self) -> Resultant<Triads> {
        let (r, t, f) = (self.root().into(), self.third().into(), self.fifth().into());

        if Fifths::Perfect * r == f {
            if is_major_third(r, t) {
                return Ok(Triads::Major);
            } else {
                return Ok(Triads::Minor);
            }
        } else {
            if is_major_third(r, t) && is_major_third(t, f) {
                return Ok(Triads::Augmented);
            } else if is_minor_third(r, t) && is_minor_third(t, f) {
                return Ok(Triads::Diminshed);
            }
            Err("Failed to find the required relationships...".to_string())
        }
    }
    /// [Triadic::config] Create a new [Configuration] with the [Triad] as its alphabet
    fn config(&self) -> Configuration<Note> {
        Configuration::norm(Tape::new(self.chord())).unwrap()
    }
    /// [Triadic::machine] Tries to create a [Machine] running the given [Program] with a default set to the triad's root
    fn machine(&self, program: Program<Note>) -> Resultant<Machine<Note>> {
        Machine::new(self.root(), program)
    }
    /// [Triadic::is_valid] A method for establishing the validity of the given notes
    fn is_valid(&self) -> bool {
        self.classify().is_ok()
    }
    fn fifth(&self) -> Note;
    fn root(&self) -> Note;
    fn third(&self) -> Note;
    fn triad(&self) -> &Self
    where
        Self: Sized,
    {
        &self
    }
}

impl Triadic for (i64, i64, i64) {
    fn fifth(&self) -> Note {
        self.2.into()
    }

    fn root(&self) -> Note {
        self.0.into()
    }

    fn third(&self) -> Note {
        self.1.into()
    }
}

impl Triadic for (Note, Note, Note) {
    fn fifth(&self) -> Note {
        self.2.clone()
    }

    fn root(&self) -> Note {
        self.0.clone()
    }

    fn third(&self) -> Note {
        self.1.clone()
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum Triads {
    Augmented, // If the root -> third is major and if third -> fifth is major
    Diminshed, // If the root -> third is minor and if third -> fifth is minor
    #[default]
    Major, // If the root -> third is major and if third -> fifth is minor
    Minor,     // If the root -> third is minor and if third -> fifth is major
}

/// [Triad] is a set of three [Note], the root, third, and fifth.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Triad(Note, Note, Note);

impl Triad {
    pub fn new(root: Note, class: Triads) -> Self {
        let (a, b) = Thirds::compute_both(root.clone());
        match class {
            Triads::Augmented => Self(root, a.clone(), Thirds::Major * a),
            Triads::Diminshed => Self(root, b.clone(), Thirds::Minor * b),
            Triads::Major => Self(root, a.clone(), Thirds::Minor * a),
            Triads::Minor => Self(root, b.clone(), Thirds::Major * b),
        }
    }
}

impl Symbolic for Triad {}

impl Triadic for Triad {
    fn fifth(&self) -> Note {
        self.2.clone()
    }

    fn root(&self) -> Note {
        self.0.clone()
    }

    fn third(&self) -> Note {
        self.1.clone()
    }
}

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

impl TryFrom<(i64, i64, i64)> for Triad {
    type Error = String;
    fn try_from(data: (i64, i64, i64)) -> Result<Triad, Self::Error> {
        let notes: (Note, Note, Note) = (data.0.into(), data.1.into(), data.2.into());
        Triad::try_from(notes)
    }
}

impl From<Triad> for (i64, i64, i64) {
    fn from(d: Triad) -> (i64, i64, i64) {
        (d.0.into(), d.1.into(), d.2.into())
    }
}

impl TryFrom<(Note, Note, Note)> for Triad {
    type Error = String;

    fn try_from(data: (Note, Note, Note)) -> Result<Triad, Self::Error> {
        let args = vec![data.0, data.1, data.2];
        for i in 0..args.len() {
            let tmp = [(i + 1) % args.len(), (i + 2) % args.len()];
            for j in 0..tmp.len() {
                let (a, b, c) = (
                    args[i].clone(),
                    args[tmp[j]].clone(),
                    args[tmp[(j + 1) % tmp.len()]].clone(),
                );
                // Creates a triad if the two intervals of [root, third], [third, fifth] are both considered thirds
                if is_third(a.clone().into(), b.clone().into())
                    && is_third(b.clone().into(), c.clone().into())
                {
                    return Ok(Triad(a, b, c));
                }
            }
        }
        Err("Failed to find the required relationships within the given notes...".to_string())
    }
}

impl From<Triad> for (Note, Note, Note) {
    fn from(d: Triad) -> (Note, Note, Note) {
        (d.0.clone(), d.1.clone(), d.2.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triad() {
        let a = Triad::new(0.into(), Triads::Major);
        let tmp: (i64, i64, i64) = a.clone().into();
        assert_eq!(tmp, (0, 4, 7));
        let b = Triad::try_from((11, 4, 7));

        assert!(b.is_ok());
        assert_ne!(a, b.unwrap())
    }
}
