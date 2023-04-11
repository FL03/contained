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
use super::Tonnetz;
use crate::intervals::{Thirds, Fifths};
use crate::neo::triads::*;
use crate::{intervals::Interval, Note};
use decanter::prelude::H256;
use petgraph::graph::UnGraph;
use std::sync::{Arc, Mutex};

pub enum ClusterEvent {
    TriadAdded { id: Note },
    TriadRemoved { id: Note },
    None,
}

pub struct Boundary {
    pub id: H256, // the id of the triad that is the boundary
    pub interval: Interval,
}

#[derive(Clone, Debug, Default)]
pub struct Cluster {
    cluster: UnGraph<Note, Interval>,
    scope: Arc<Mutex<Triad>>,
}

impl Cluster {}

impl std::fmt::Display for Cluster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.cluster)
    }
}

impl Tonnetz for Cluster {
    fn scope(&self) -> &Arc<Mutex<Triad>> {
        &self.scope
    }

    fn tonnetz(&self) -> &UnGraph<Note, Interval> {
        &self.cluster
    }

    fn tonnetz_mut(&mut self) -> &mut UnGraph<Note, Interval> {
        &mut self.cluster
    }
}

impl From<Triad> for Cluster {
    fn from(triad: Triad) -> Self {
        let (rt, tf, rf): (Thirds, Thirds, Fifths) = triad.intervals();
        let mut cluster = UnGraph::new_undirected();
        cluster.reserve_nodes(crate::MODULUS as usize);
        let root = cluster.add_node(triad.root());
        let third = cluster.add_node(triad.third());
        let fifth = cluster.add_node(triad.fifth());
        cluster.add_edge(root, third, rt.into());
        cluster.add_edge(third, fifth, tf.into());
        cluster.add_edge(root, fifth, rf.into());
        Self {
            cluster: cluster,
            scope: Arc::new(Mutex::new(triad)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MODULUS;

    #[test]
    fn test_cluster() {
        let triad = Triad::new(0.into(), TriadClass::Major);

        let mut cluster = Cluster::from(triad.clone());
        assert!(cluster.fulfilled() == false);
        for i in 1..MODULUS {
            cluster.insert(Triad::new(i.into(), TriadClass::Major));
        }
        // assert!(cluster.fulfilled() == true);
        for class in [
            TriadClass::Minor,
            TriadClass::Augmented,
            TriadClass::Diminished,
        ] {
            for i in 0..MODULUS {
                cluster.insert(Triad::new(i.into(), class));
            }
        }
        // assert!(cluster.fulfilled() == true);
    }
}
