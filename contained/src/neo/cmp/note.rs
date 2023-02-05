/*
    Appellation: note <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        A note is a symbolic representation of the duration and pitch of a tone

        For our purposes, let a pitch represent a unique behavior and a pitch-class be a system addressed by the behavior

*/
use super::{Epoch, PitchClass};
use scsys::prelude::SerdeDisplay;
use serde::{Deserialize, Serialize};

pub trait Notable {
    fn pitch(&self) -> &PitchClass
    where
        Self: Sized;
    fn epoch(&self) -> &Epoch
    where
        Self: Sized;
}

/// A [Note] consists of some [Pitch] and a [Epoch] which indicates a start time and optionally signals a duration
#[derive(
    Clone,
    Debug,
    Default,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    SerdeDisplay,
    Serialize,
)]
pub struct Note(PitchClass, Epoch);

impl Note {
    pub fn new(pitch: PitchClass, epoch: Epoch) -> Self {
        Self(pitch, epoch)
    }
}
