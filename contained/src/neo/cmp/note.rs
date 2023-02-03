/*
    Appellation: note <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        A note is a symbolic representation of the duration and pitch of a tone

        For our purposes, let a pitch represent a unique behavior and a pitch-class be a system addressed by the behavior

*/
use super::{Epoch, Pitch};
use serde::{Deserialize, Serialize};

/// A [Note] consists of some [Pitch] and a [Epoch] which indicates a start time and optionally signals a duration
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Note<P: Pitch>(P, Epoch);
