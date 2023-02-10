/*
    Appellation: note <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        A note is a symbolic representation of the duration and pitch of a tone

        For our purposes, let a pitch represent a unique behavior and a pitch-class be a system addressed by the behavior

        If a note is simply a symbolic representation of a pitch than we can assume all pitches to be modulus of 12
*/
use crate::neo::cmp::{Epoch, Pitch, PitchClass};
use crate::turing::Symbolic;
use serde::{Deserialize, Serialize};

pub struct ASPN(PitchClass, i64);

impl ASPN {
    pub fn new(pitch: PitchClass, octave: i64) -> Self {
        Self(pitch, octave)
    }
}

/// A [Note] consists of some [PitchClass] and an [Option<Epoch>] which indicates a start time and optionally signals a duration
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialOrd, Serialize)]
pub struct Note(PitchClass, Option<Epoch>);

impl Note {
    pub fn new(pitch: PitchClass, epoch: Option<Epoch>) -> Self {
        Self(pitch, epoch)
    }
    pub fn pitch(&self) -> Pitch {
        self.0.clone().into()
    }
    pub fn epoch(&self) -> &Option<Epoch> {
        &self.1
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

impl From<i64> for Note {
    fn from(d: i64) -> Note {
        Note::new(PitchClass::from(d), None)
    }
}
