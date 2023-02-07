/*
    Appellation: chord <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A chord is any set of notes played simultaneously; for our considerations, allow a chord to represent the alphabet of a Turing machine or automata.

*/
use crate::neo::cmp::{Note, Pitch};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

pub trait ArrayLike {
    type Data;

    fn content(&self) -> &Vec<Self::Data>;
    fn mut_content(&mut self) -> &mut Vec<Self::Data>;
    fn insert(&mut self, index: usize, elem: Self::Data) {
        self.mut_content().insert(index, elem)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Chord(Vec<Pitch>);

impl Chord {
    pub fn new(chord: impl IntoIterator<Item = Pitch>) -> Self {
        Self(Vec::from_iter(chord))
    }
    pub fn chord(&self) -> &Vec<Pitch> {
        &self.0
    }
    
    pub fn append(&mut self, elem: &mut Vec<Pitch>) {
        self.0.append(elem);
    }
    pub fn extend(&mut self, elem: impl IntoIterator<Item = Pitch>) {
        self.0.extend(Vec::from_iter(elem));
    }
    pub fn insert(&mut self, index: usize, elem: Pitch) {
        self.0.insert(index, elem);
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

    #[test]
    fn test_chord_factors() {
        let a = ChordFactors::default();
        assert_eq!(a.clone(), ChordFactors::Root(Default::default()))
    }
}
