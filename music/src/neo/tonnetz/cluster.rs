/*
    Appellation: cluster <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        This module is dedicated to the proposed harmonic computational fabric;
        A cluster is a type of tonnetz that is used to orchestrate a set of local or detached triadic machines.
        The cluster is an undirected, circular graph where each node is a note which is connected to 6 other nodes.

        If a tonnetz is a topological computer, then a cluster is a topological computer that is used to orchestrate a set of topological computers.
        Locally, a tonnetz is typically fragemented only persisting as many triads as the host device allows for. However, as a network the cluster
        glues together these framents into a single, cohesive, and complete experience orchestrated according to a single originator.
*/
use crate::neo::triads::*;
use crate::{intervals::Interval, Note, MODULUS};
use algae::graph::{Graph, UndirectedGraph};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Default)]
pub struct Cluster {
    cluster: UndirectedGraph<Note, Interval>,
    scope: Arc<Mutex<Triad>>,
}

impl Cluster {
    pub fn fulfilled(&self) -> bool {
        self.cluster.nodes().len() == MODULUS as usize
    }
    pub fn insert(&mut self, triad: Triad) {
        // determine the intervals used to create the given triad
        let (a, b, c): (Interval, Interval, Interval) = triad.clone().into();

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

impl std::fmt::Display for Cluster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.cluster)
    }
}

impl From<Triad> for Cluster {
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

    #[test]
    fn test_cluster() {
        let triad = Triad::new(0.into(), Triads::Major);

        let mut cluster = Cluster::from(triad.clone());
        assert!(cluster.fulfilled() == false);
        for i in 1..MODULUS {
            cluster.insert(Triad::new(i.into(), Triads::Major));
        }
        assert!(cluster.fulfilled() == true);
        for class in [Triads::Minor, Triads::Augmented, Triads::Diminished] {
            for i in 0..MODULUS {
                cluster.insert(Triad::new(i.into(), class));
            }
        }
        assert!(cluster.fulfilled() == true);
    }
}
