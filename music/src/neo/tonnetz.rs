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
use super::{
    triads::{Triad, Triads},
    Boundary,
};
use crate::{intervals::Interval, Notable, Note};
use algae::graph::{Graph, UndirectedGraph};
use decanter::prelude::{Hashable, H256};
use std::sync::Arc;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Tonnetz<N: Notable = Note> {
    cluster: UndirectedGraph<N, H256>,
    scope: Arc<Triad<N>>,
}

impl<N: Notable> Tonnetz<N> {
    pub fn fulfilled(&self) -> bool {
        self.cluster.nodes().len() == crate::MODULUS as usize
    }
    pub fn insert(&mut self, triad: Triad<N>) {
        let (r, t, f): (N, N, N) = triad.clone().into();
        let (rt, tf, rf): (Interval, Interval, Interval) = Triads::try_from(triad.clone())
            .expect("Invalid triad")
            .into();
        let seed = triad.hash();

        self.cluster
            .add_edge((r.clone(), t.clone(), Boundary::new(rt, seed).hash()));
        self.cluster
            .add_edge((t, f.clone(), Boundary::new(tf, seed).hash()));
        self.cluster
            .add_edge((r, f, Boundary::new(rf, seed).hash()));
    }
}

impl<N: Notable> std::fmt::Display for Tonnetz<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.cluster)
    }
}

impl<N: Notable> From<Triad<N>> for Tonnetz<N> {
    fn from(triad: Triad<N>) -> Self {
        let (r, t, f): (N, N, N) = triad.clone().into();
        let (rt, tf, rf): (Interval, Interval, Interval) = Triads::try_from(triad.clone())
            .expect("Invalid triad")
            .into();
        let seed = triad.hash();

        let mut cluster = UndirectedGraph::new();
        cluster.add_edge((r.clone(), t.clone(), Boundary::new(rt, seed).hash()));
        cluster.add_edge((t, f.clone(), Boundary::new(tf, seed).hash()));
        cluster.add_edge((r, f, Boundary::new(rf, seed).hash()));
        Self {
            cluster: cluster.clone(),
            scope: Arc::new(triad),
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

        let mut tonnetz = Tonnetz::from(triad.clone());
        assert!(tonnetz.fulfilled() == false);
        for i in 1..crate::MODULUS {
            tonnetz.insert(Triad::<Note>::new(i.into(), Triads::Major));
        }
        assert!(tonnetz.fulfilled() == true);
    }
}
