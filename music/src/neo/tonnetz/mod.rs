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
/// # Tonnetz
/// 
/// A tonnetz is a type of topological computer that is used to orchestrate a set of local or detached triadic machines.
/// A single tonnetz is considered to be any set of connected, non-repeating traids. Repeating triadic structures are
/// used to create additional layers and can be used to enact more complex workloads. 
/// 
pub use self::cluster::*;

mod cluster;

use crate::neo::triads::*;
use crate::{intervals::Interval, Note, MODULUS};
use algae::graph::{Graph, UndirectedGraph};
use decanter::prelude::{Hashable, Iter, H256};
use std::sync::{Arc, Mutex};

pub trait Link: Hashable {
    /// [Link::bridge] is used to synchronize the activties of two different triads; required to seperated by a single LPR transformation
    fn bridge(&self, with: impl Hashable) -> H256 {
        let mut iter = Iter::new();
        iter.extend(vec![self.hash(), with.hash()]);
        iter.hash()
    }
    fn interval(&self) -> Interval;
}

pub trait Tonnetz {
    fn fulfilled(&self) -> bool {
        self.tonnetz().nodes().len() == MODULUS as usize
    }
    fn insert(&mut self, triad: Triad) {
        // determine the intervals used to create the given triad
        let (a, b, c): (Interval, Interval, Interval) = triad.try_into().expect("Invalid triad");
        self.tonnetz_mut()
            .add_edge((triad.root(), triad.third(), a).into());
        self.tonnetz_mut()
            .add_edge((triad.third(), triad.fifth(), b).into());
        self.tonnetz_mut()
            .add_edge((triad.root(), triad.fifth(), c).into());
    }
    fn scope(&self) -> &Arc<Mutex<Triad>>;
    fn tonnetz(&self) -> &UndirectedGraph<Note, Interval>;
    fn tonnetz_mut(&mut self) -> &mut UndirectedGraph<Note, Interval>;
}
