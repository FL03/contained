/*
    Appellation: note <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        A note is a symbolic representation of the duration and pitch of a tone

        For our purposes, let a pitch represent a unique behavior and a pitch-class be a system addressed by the behavior

*/
use scsys::prelude::Timestamp;
use serde::{Deserialize, Serialize};

///
pub trait Pitch: Clone + Default + ToString {}

/// An [Epoch] consists of a start time and optionally, a duration (seconds). If None, the system assumes an infinite duration
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Epoch(i64, Option<i64>);

impl Default for Epoch {
    fn default() -> Self {
        Self(Timestamp::default().into(), Some(1000))
    }
}

/// A [Note] consists of some [Pitch] and a [Epoch] which indicates a start time and optionally signals a duration
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Note<P: Pitch>(P, Epoch);
