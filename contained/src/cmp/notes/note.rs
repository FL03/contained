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
use crate::cmp::{Notable, Pitch, PitchClass};
use crate::turing::Symbolic;
use serde::{Deserialize, Serialize};

pub struct ASPN(PitchClass, i64);

impl ASPN {
    pub fn new(pitch: PitchClass, octave: i64) -> Self {
        Self(pitch, octave)
    }
    pub fn class(&self) -> &PitchClass {
        &self.0
    }
    pub fn octave(&self) -> i64 {
        self.1
    }
}

/// A [Note] consists of some [PitchClass] and an [Option<Epoch>] which indicates a start time and optionally signals a duration
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialOrd, Serialize)]
pub struct Note(PitchClass, i64);

impl Note {
    pub fn new(class: PitchClass, octave: Option<i64>) -> Self {
        Self(class, octave.unwrap_or(1))
    }
    pub fn octave(&self) -> i64 {
        self.1
    }
}

impl Notable for Note {
    fn pitch(&self) -> Pitch {
        self.0.clone().into()
    }
}

impl Symbolic for Note {}

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Note> for i64 {
    fn from(data: Note) -> i64 {
        let pitch: Pitch = data.into();
        pitch.into()
    }
}

impl From<Note> for Pitch {
    fn from(data: Note) -> Pitch {
        data.0.into()
    }
}

impl From<Pitch> for Note {
    fn from(data: Pitch) -> Note {
        Note::new(data.into(), None)
    }
}

impl From<i64> for Note {
    fn from(d: i64) -> Note {
        Note::new(PitchClass::from(d), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmp::{NaturalNote, Notable};

    #[test]
    fn test_notes() {
        let a = Note::new(PitchClass::Natural(NaturalNote::C), None);
        assert_eq!(a.pitch(), 0.into());
        assert_eq!(a.octave(), 1);
    }
}
