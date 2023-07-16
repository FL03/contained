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
//! # Tonnetz
//!
//! A tonnetz is a type of topological computer that is used to orchestrate a set of local or detached triadic machines.
//! A single tonnetz is considered to be any set of connected, non-repeating traids. Repeating triadic structures are
//! used to create additional layers and can be used to enact more complex workloads.
pub use self::cluster::*;

mod cluster;

use crate::neo::triads::*;
use crate::{intervals::Interval, Note, MODULUS};
use decanter::prelude::{Hashable, Iter, H256};
use petgraph::graph::{DefaultIx, NodeIndex};
use petgraph::{Graph, Undirected};

pub type TonnetzGraph<Idx = DefaultIx> = Graph<Note, Interval, Undirected, Idx>;

pub trait Link: Hashable {
    /// [Link::bridge] is used to synchronize the activties of two different triads; required to seperated by a single LPR transformation
    fn bridge(&self, with: impl Hashable) -> H256 {
        let mut iter = Iter::new();
        iter.extend(vec![self.hash(), with.hash()]);
        iter.hash()
    }
    fn interval(&self) -> Interval;
}

pub trait TonnetzSpec {
    const N: usize = MODULUS as usize;

    fn add_node(&mut self, note: Note) -> NodeIndex {
        // Check if the node already exists
        if let Some(index) = self
            .store()
            .node_indices()
            .find(|&i| self.store()[i] == note)
        {
            return index;
        }

        // Node doesn't exist, add it and return the new NodeIndex
        self.store_mut().add_node(note)
        // self.tonnetz_mut().add_node(note)
    }
    fn fulfilled(&self) -> bool {
        self.store().node_count() == Self::N
    }
    fn insert(&mut self, triad: Triad) {
        use ChordFactor::*;
        // determine the intervals used to create the given triad
        let (a, b, c): (Interval, Interval, Interval) = triad.try_into().expect("Invalid triad");

        let r = self.add_node(triad[Root]);
        let t = self.add_node(triad[Third]);
        let f = self.add_node(triad[Fifth]);
        self.store_mut()
            .extend_with_edges([(r, t, a), (t, f, b), (r, f, c)]);
    }
    fn scope(&self) -> Triad;
    fn store(&self) -> &TonnetzGraph;
    fn store_mut(&mut self) -> &mut TonnetzGraph;
}
