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
use super::{TonnetzGraph, TonnetzSpec};
use crate::neo::triads::*;
use crate::prelude::{Interval, Note, LPR};
use decanter::prelude::H256;
use petgraph::{Graph, Undirected};
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
    scope: Arc<Mutex<Triad>>,
    store: TonnetzGraph,
}

impl Cluster {}

impl std::fmt::Display for Cluster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.store)
    }
}

impl TonnetzSpec for Cluster {
    fn scope(&self) -> Triad {
        *self.scope.lock().unwrap()
    }

    fn store(&self) -> &TonnetzGraph {
        &self.store
    }

    fn store_mut(&mut self) -> &mut TonnetzGraph {
        &mut self.store
    }
}

impl AsRef<TonnetzGraph> for Cluster {
    fn as_ref(&self) -> &TonnetzGraph {
        &self.store
    }
}

impl AsMut<TonnetzGraph> for Cluster {
    fn as_mut(&mut self) -> &mut TonnetzGraph {
        &mut self.store
    }
}

impl From<Triad> for Cluster {
    fn from(triad: Triad) -> Self {
        let (rt, tf, rf): (Interval, Interval, Interval) = triad.try_into().expect("Invalid triad");
        let mut cluster = Graph::<Note, Interval, Undirected>::new_undirected();

        let r = cluster.add_node(triad.root());
        let t = cluster.add_node(triad.third());
        let f = cluster.add_node(triad.fifth());
        cluster.extend_with_edges([(r, t, rt), (t, f, tf), (r, f, rf)]);
        Self {
            store: cluster,
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
        eprintln!("{:?}", cluster.store());
        assert!(cluster.fulfilled());
        for class in [Triads::Minor, Triads::Augmented, Triads::Diminished] {
            for i in 0..MODULUS {
                cluster.insert(Triad::new(i.into(), class));
            }
        }
        assert!(cluster.fulfilled());
    }
}
