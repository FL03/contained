/*
    Appellation: tonnetz <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        A tonnetz can be any set of connected, non-repeating triads. The tonnetz is essentially a topological computer created by gluing together several triadic machines together.
        The tonnetz is an undirected, circular graph where each node is a note which is connected to 6 other nodes.

        To find the six related notes, one simply must find the thirds and perfect fifth that lie above and below the given node. For example,
            Note(C) -> ((3, -3), (4, -4), (7, -7))
                (Minor Third)   +/- 3 -> (D# / Eb, A)
                (Major Third)   +/- 4 -> (E, G# / Ab)
                (Perfect Fifth) +/- 7 -> (G, F)
*/
use super::{triads::Triad, LPR};
use crate::{Interval, Notable, Note};
use contained_core::graphs::{Graph, UndirectedGraph};
use std::{collections::BTreeSet, sync::Arc};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Tonnetz<N: Notable = Note> {
    cluster: UndirectedGraph<N, Interval>,
    scope: Arc<Triad<N>>,
}

impl<N: Notable> Tonnetz<N> {
    pub fn new(cluster: UndirectedGraph<N, Interval>, scope: Arc<Triad<N>>) -> Self {
        Self { cluster, scope }
    }
}

impl<N: Notable> std::fmt::Display for Tonnetz<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.cluster)
    }
}

impl<N: Notable> From<Triad<N>> for Tonnetz<N> {
    fn from(triad: Triad<N>) -> Self {
        Self {
            cluster: Default::default(),
            scope: triad.into(),
        }
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
