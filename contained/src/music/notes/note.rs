/*
    Appellation: note <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        A note is a symbolic representation of the duration and pitch of a tone;
        The enharmonic nature of musical notation enables us to create a system entirely
        dependent upon the modulus of the Pitch rather than the specific symbol.
        That being said, we will also adopt a note representation similar to that of the
        American Scientific Pitch Notation which denotes a certain octave for the given pitch-class.
*/
use crate::actors::turing::Symbolic;
use crate::music::{Gradient, Notable, PitchClass};
use serde::{Deserialize, Serialize};

/// A [Note] is simply a wrapper for a [PitchClass], providing additional information such as an octave ([i64])
/// This type of musical notation is adopted from the American Scientific Pitch Notation
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Note(PitchClass, i64);

impl Note {
    pub fn new(class: PitchClass, octave: Option<i64>) -> Self {
        Self(class, octave.unwrap_or(1))
    }
    pub fn octave(&self) -> i64 {
        self.1
    }
}

impl Gradient for Note {
    fn class(&self) -> PitchClass {
        self.0.clone()
    }
}

impl Notable for Note {}

impl Symbolic for Note {}

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.0, self.1)
    }
}

impl From<i64> for Note {
    fn from(data: i64) -> Note {
        Note::new(PitchClass::from(data), None)
    }
}

impl From<Note> for i64 {
    fn from(data: Note) -> i64 {
        data.class().into()
    }
}

impl<P: Gradient> From<&P> for Note {
    fn from(d: &P) -> Note {
        Note::new(d.class(), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::music::{Gradient, NaturalNote};

    #[test]
    fn test_notes() {
        let a = Note::new(PitchClass::Natural(NaturalNote::C), None);
        assert_eq!(a.pitch(), 0);
        assert_eq!(a.octave(), 1);
    }
}