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
use crate::neo::{cmp::Note, LPR, SEMITONE};
use crate::turing::{Configuration, Symbolic, Tape};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

pub trait Triadic {
    fn fifth(&self) -> Note;
    fn root(&self) -> Note;
    fn third(&self) -> Note;
    fn triad(&self) -> Triad;
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
pub enum Thirds {
    #[default]
    Major = 0,
    Minor = 1,
}

impl Thirds {
    pub fn find_third(&self, note: Note) -> Note {
        let a: i64 = note.pitch().into();
        match self {
            Self::Major => Note::from(major_third(a)),
            Self::Minor => Note::from(minor_third(a)),
        }
    }
}

pub fn is_third(a: i64, b: i64) -> bool {
    if is_major_third(a, b) || is_minor_third(a, b) {
        return true;
    }
    false
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

pub fn is_major_third(a: i64, b: i64) -> bool {
    if major_third(a) == b {
        return true;
    }
    false
}

pub fn is_minor_third(a: i64, b: i64) -> bool {
    if minor_third(a) == b {
        return true;
    }
    false
}

pub fn major_third(pitch: i64) -> i64 {
    (pitch + 4 * SEMITONE as i64) % 12
}

pub fn minor_third(pitch: i64) -> i64 {
    (pitch + 3 * SEMITONE as i64) % 12
}

pub fn perfect_fifth(pitch: i64) -> i64 {
    (pitch + 7) % 12
}

pub fn classify_triad(triad: &Triad) -> Option<Triads> {
    let (r, t, f) = triad.clone().into();

    if is_major_third(r, t) && is_minor_third(t, f) {
        return Some(Triads::Major);
    } else if is_minor_third(r, t) && is_major_third(t, f) {
        return Some(Triads::Minor);
    } else if is_major_third(r, t) && is_major_third(t, f) {
        return Some(Triads::Augmented);
    } else if is_minor_third(r, t) && is_minor_third(t, f) {
        return Some(Triads::Diminshed);
    } else {
        return None;
    }
}

/// [Triad] is a set of three [Note], the root, third, and fifth.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Triad(Note, Note, Note);

impl Triad {
    pub fn new(root: Note, third: Note, fifth: Note) -> Self {
        Self(root, third, fifth)
    }
    pub fn build(root: Note, class: Triads) -> Self {
        let pitch: i64 = root.pitch().into();
        let (third_maj, third_minor) = (major_third(pitch), minor_third(pitch));
        match class {
            Triads::Augmented => Self::new(
                root,
                Note::from(third_maj),
                Note::from(major_third(third_maj)),
            ),
            Triads::Diminshed => Self::new(
                root,
                Note::from(third_minor),
                Note::from(minor_third(third_minor)),
            ),
            Triads::Major => Self::new(
                root,
                Note::from(third_maj),
                Note::from(minor_third(third_maj)),
            ),
            Triads::Minor => Self::new(
                root,
                Note::from(third_minor),
                Note::from(major_third(third_minor)),
            ),
        }
    }
    /// Create a new [Configuration] with the [Triad] as its alphabet
    pub fn config(&self) -> Configuration<Note> {
        Configuration::norm(Tape::new(self.clone())).unwrap()
    }
    /// A method for establishing the validity of the given notes
    pub fn is_valid(&self) -> bool {
        if classify_triad(&self).is_some() {
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
        let a = Triad::from((0, 3, 6));
        let b = Triad::from((2, 6, 9));
        assert!(a.is_valid());
        assert_eq!(classify_triad(&a), Some(Triads::Diminshed));
        assert_eq!(&a, &Triad::build(Note::from(0), Triads::Diminshed));
        assert!(b.is_valid());
        assert_eq!(classify_triad(&b), Some(Triads::Major));
        assert_ne!(a, b)
    }
}
