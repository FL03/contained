/*
    Appellation: chord <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A chord is any set of notes played simultaneously; for our considerations, allow a chord to represent the alphabet of a Turing machine or automata.

*/
use crate::neo::cmp::{Epoch, Note, PitchClass};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Chord(Vec<PitchClass>, Epoch);

pub trait ChordFactor {
    fn factor(&self) -> &ChordFactors
    where
        Self: Sized;
    fn note(&self) -> &Note
    where
        Self: Sized;
}

#[derive(
    Clone,
    Debug,
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
    SmartDefault,
)]
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum ChordFactors {
    #[default]
    Root(Note) = 0,
    Third(Note) = 1,
    Fifth(Note) = 2,
}

pub type Root = Note;
pub type Third = Note;
pub type Fifth = Note;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chord_factors() {
        let a = ChordFactors::default();
        assert_eq!(a.clone(), ChordFactors::Root(Default::default()))
    }
}
