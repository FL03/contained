/*
    Appellation: tonnetz <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        A tonnetz can be any set of connected, non-repeating triads. The tonnetz is essentially a topological computer created by gluing together several triadic machines together.
        The tonnetz is an undirected, circular graph where each node is a note which is connected to 6 other nodes. There are a total of six connections for each note because the
        graphical interpretation of the tonnetz considers all of the possible configurations of a triad and

            Note(C) -> ((3, -3), (4, -4), (7, -7))
                +/- 3 -> (D# / Eb, A)
                +/- 4 -> (E, G# / Ab)
                +/- 7 -> (G, F)
*/
use super::{triads::Triad, LPR};
use crate::{Notable, Note};
use std::{collections::BTreeSet, sync::Arc};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tonnetz<N: Notable = Note> {
    cluster: BTreeSet<Triad<N>>,
}

impl<N: Notable> Tonnetz<N> {
    pub fn new(cluster: BTreeSet<Triad<N>>) -> Self {
        Self { cluster }
    }
}

impl<N: Notable> std::fmt::Display for Tonnetz<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cluster.clone().first().unwrap())
    }
}

impl<N: Notable> From<Triad<N>> for Tonnetz<N> {
    fn from(triad: Triad<N>) -> Tonnetz<N> {
        Tonnetz::<N>::new(BTreeSet::from([triad]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neo::triads::{Triad, Triads};
    use crate::Note;

    #[test]
    fn test_tonnetz() {
        let triad = Triad::<Note>::new(0.into(), Triads::Major);

        let mut a = Tonnetz::from(triad.clone());
    }
}
