/*
    Appellation: cluster <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        The cluster is an undirected, circular graph where each node is a note which is connected to 6 other nodes.

        If a tonnetz is a topological computer, then a cluster is a topological computer that is used to orchestrate a set of topological computers.
        Locally, a tonnetz is typically fragemented only persisting as many triads as the host device allows for. However, as a network the cluster
        glues together these framents into a single, cohesive, and complete experience orchestrated according to a single originator.

        Each triad persisted is required to maintian a set of invariants that are used to determine the state of the cluster.
        If each edge in a traditional tonnetz is the interval between the two notes, than each edge in the cluster describes a type of seed value that encodes some information about the triad.
*/
//! # Cluster
//! 
//! A cluster is a type of tonnetz that is used to orchestrate a set of local or detached triadic machines.
use super::Tonnetz;
use crate::intervals::{Fifths, Thirds};
use crate::neo::triads::*;
use crate::{intervals::Interval, neo::LPR, Note};
use algae::graph::{Graph, UndirectedGraph};
use decanter::prelude::H256;
use std::sync::{Arc, Mutex};

pub enum ClusterEvent {
    Applied(LPR),
    Registered { triad: Triad },
    Unregistered { triad: Triad },
}

pub struct Boundary {
    pub id: H256, // the id of the triad that is the boundary
    pub interval: Interval,
}

#[derive(Clone, Debug, Default)]
pub struct Cluster {
    cluster: UndirectedGraph<Note, Interval>,
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

    fn tonnetz(&self) -> &UndirectedGraph<Note, Interval> {
        &self.cluster
    }

    fn tonnetz_mut(&mut self) -> &mut UndirectedGraph<Note, Interval> {
        &mut self.cluster
    }
}

impl From<Triad> for Cluster {
    fn from(triad: Triad) -> Self {
        let (rt, tf, rf): (Thirds, Thirds, Fifths) = triad.intervals();
        let mut cluster = UndirectedGraph::with_capacity(crate::MODULUS as usize);

        cluster.add_edge((triad.root(), triad.third(), rt.into()).into());
        cluster.add_edge((triad.third(), triad.fifth(), tf.into()).into());
        cluster.add_edge((triad.root(), triad.fifth(), rf.into()).into());
        Self {
            cluster,
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
        let triad = Triad::new(0.into(), Triads::Major);

        let mut cluster = Cluster::from(triad);
        assert!(!cluster.fulfilled());
        for i in 1..MODULUS {
            cluster.insert(Triad::new(i.into(), Triads::Major));
        }
        eprintln!("{:?}", cluster.tonnetz().nodes());
        assert!(cluster.fulfilled());
        for class in [Triads::Minor, Triads::Augmented, Triads::Diminished] {
            for i in 0..MODULUS {
                cluster.insert(Triad::new(i.into(), class));
            }
        }
        assert!(cluster.fulfilled());
    }
}
