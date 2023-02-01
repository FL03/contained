/*
    Appellation: note <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        A note is a symbolic representation of the duration and pitch of a tone
*/
use serde::{Deserialize, Serialize};

///
pub trait Pitch: Clone + Default + ToString {}

/// The duration or epoch of a note is a set of timestamps
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Epoch(i64, i64);

///
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Note<P: Pitch>(P, i64);
