/*
    Appellation: chord <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A chord is any set of notes played simultaneously; for our considerations, allow a chord to represent the alphabet of a Turing machine or automata.

*/
use crate::neo::cmp::{Note, Pitch};
use crate::ArrayLike;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

/// [Chord] is a wrapper for a [Vec] of [Pitch]
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Chord(Vec<Pitch>);

impl Chord {
    pub fn new(chord: impl IntoIterator<Item = Pitch>) -> Self {
        Self(Vec::from_iter(chord))
    }
    pub fn chord(&self) -> &Self {
        self
    }
}

impl ArrayLike for Chord {
    type Data = Pitch;

    fn content(&self) -> &Vec<Self::Data> {
        &self.0
    }

    fn mut_content(&mut self) -> &mut Vec<Self::Data> {
        &mut self.0
    }
}

impl IntoIterator for Chord {
    type Item = Pitch;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

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
    use crate::neo::cmp::Pitch;

    #[test]
    fn test_chords() {
        let a = vec![Pitch::from(0), Pitch::from(3), Pitch::from(8)];
        let mut b = Chord::default();
        assert!(b.is_empty());
        b.append(&mut a.clone());
        assert_eq!(b.len(), 3);
    }

    #[test]
    fn test_chord_factors() {
        let a = ChordFactors::default();
        assert_eq!(a, ChordFactors::Root(Default::default()))
    }
}
