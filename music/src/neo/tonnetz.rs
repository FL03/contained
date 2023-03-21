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
use super::triads::{Triad, Triadic};
use crate::{intervals::Interval, Note, MODULUS};
use algae::graph::{Graph, UndirectedGraph};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Default)]
pub struct Tonnetz {
    cluster: UndirectedGraph<Note, Interval>,
    scope: Arc<Mutex<Triad>>,
}

impl Tonnetz {
    pub fn fulfilled(&self) -> bool {
        self.cluster.nodes().len() == MODULUS as usize
    }
    pub fn insert(&mut self, triad: Triad) {
        // determine the intervals used to create the given triad
        let (a, b, c): (Interval, Interval, Interval) =
            triad.clone().try_into().expect("Invalid triad");

        self.cluster
            .add_edge((triad.root(), triad.third(), a).into());
        self.cluster
            .add_edge((triad.third(), triad.fifth(), b).into());
        self.cluster
            .add_edge((triad.root(), triad.fifth(), c).into());
    }
    pub fn scope(&self) -> &Arc<Mutex<Triad>> {
        &self.scope
    }
}

impl std::fmt::Display for Tonnetz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.cluster)
    }
}

impl From<Triad> for Tonnetz {
    fn from(triad: Triad) -> Self {
        // determine the intervals used to create the given triad
        let (a, b, c): (Interval, Interval, Interval) =
            triad.clone().try_into().expect("Invalid triad");
        let mut cluster = UndirectedGraph::with_capacity(MODULUS as usize);
        cluster.add_edge((triad.root(), triad.third(), a).into());
        cluster.add_edge((triad.third(), triad.fifth(), b).into());
        cluster.add_edge((triad.root(), triad.fifth(), c).into());
        Self {
            cluster,
            scope: Arc::new(Mutex::new(triad)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neo::triads::{Triad, Triads};
    use crate::MODULUS;

    #[test]
    fn test_tonnetz() {
        let triad = Triad::new(0.into(), Triads::Major);

        let mut tonnetz = Tonnetz::from(triad.clone());
        assert!(tonnetz.fulfilled() == false);
        for i in 1..MODULUS {
            tonnetz.insert(Triad::new(i.into(), Triads::Major));
        }
        assert!(tonnetz.fulfilled() == true);
        for class in [Triads::Minor, Triads::Augmented, Triads::Diminished] {
            for i in 0..MODULUS {
                tonnetz.insert(Triad::new(i.into(), class));
            }
        }
        assert!(tonnetz.fulfilled() == true);
    }
}
