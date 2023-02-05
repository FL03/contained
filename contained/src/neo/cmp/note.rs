/*
    Appellation: note <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        A note is a symbolic representation of the duration and pitch of a tone

        For our purposes, let a pitch represent a unique behavior and a pitch-class be a system addressed by the behavior

*/
use super::{Epoch, PitchClass};
use crate::turing::Symbolic;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub trait Notable {
    fn pitch(&self) -> &PitchClass
    where
        Self: Sized;
    fn epoch(&self) -> &Option<Epoch>
    where
        Self: Sized;
}

/// A [Note] consists of some [Pitch] and a [Epoch] which indicates a start time and optionally signals a duration
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialOrd, Serialize)]
pub struct Note(PitchClass, Option<Epoch>);

impl Note {
    pub fn new(pitch: PitchClass, epoch: Option<Epoch>) -> Self {
        Self(pitch, epoch)
    }
}

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

impl Symbolic for Note {}


impl TryFrom<&str> for Note {
    type Error = std::string::ParseError;

    fn try_from(d: &str) -> Result<Note, Self::Error> {
        match PitchClass::from_str(d) {
            Ok(v) => Ok(Note::from(v)),
            Err(_) => panic!("ParseError")
        }
    }
}

impl From<PitchClass> for Note {
    fn from(d: PitchClass) -> Note {
        Note::new(d, None)
    }
}

impl From<i64> for Note {
    fn from(d: i64) -> Note {
        let pitch: PitchClass = d.into();
        Note::from(pitch)
    }
}
