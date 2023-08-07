/*
    Appellation: misc <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Each triad is composed of three notes, called chord factors: root, third, and fifth; these are used as a means of indexing any given triad
*/
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, EnumVariantNames, IntoEnumIterator};

/// A [ChordFactor] is used as an indexer for a [super::Triad]
/// [ChordFactor::Root] is the first note, [ChordFactor::Third] is the second note, and [ChordFactor::Fifth] is the third note in a [super::Triad]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumIter,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[repr(usize)]
#[strum(serialize_all = "lowercase")]
pub enum ChordFactor {
    #[default]
    #[strum(serialize = "r", serialize = "root")]
    Root = 0,
    #[strum(serialize = "t", serialize = "third")]
    Third = 1,
    #[strum(serialize = "f", serialize = "fifth")]
    Fifth = 2,
}

impl ChordFactor {
    pub fn fifth() -> Self {
        ChordFactor::Fifth
    }
    pub fn root() -> Self {
        ChordFactor::Root
    }
    pub fn third() -> Self {
        ChordFactor::Third
    }
    pub fn factors() -> Vec<Self> {
        Self::iter().collect()
    }
    pub fn others(&self) -> Vec<Self> {
        Self::iter().filter(|x| x != self).collect()
    }
}

impl From<usize> for ChordFactor {
    fn from(x: usize) -> Self {
        match x % 3 {
            0 => ChordFactor::Root,
            1 => ChordFactor::Third,
            _ => ChordFactor::Fifth,
        }
    }
}

impl From<ChordFactor> for usize {
    fn from(x: ChordFactor) -> Self {
        x as usize
    }
}

unsafe impl petgraph::graph::IndexType for ChordFactor {
    fn new(x: usize) -> Self {
        Self::from(x)
    }

    fn index(&self) -> usize {
        *self as usize
    }

    fn max() -> Self {
        Self::Fifth
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neo::triads::{Triad, Triads};

    #[test]
    fn chord_factors() {
        use ChordFactor::*;

        let triad = Triad::new(0.into(), Triads::Major);
        assert_eq!(triad[Root], 0.into());
        assert_eq!(triad[Third], 4.into());
        assert_eq!(triad[Fifth], 7.into());
        assert_eq!(triad[Third..Fifth], vec![4.into()]);
        assert_eq!(triad[Root..Fifth], vec![0.into(), 4.into()]);
    }

    #[test]
    fn chord_factors_iter() {
        use ChordFactor::*;

        let factors = ChordFactor::factors();
        assert_eq!(factors.len(), 3);
        assert_eq!(factors[0], Root);
        assert_eq!(factors[1], Third);
        assert_eq!(factors[2], Fifth);
    }
}
