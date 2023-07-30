/*
    Appellation: graph <triads>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{ChordFactor, Triads};
use crate::prelude::{Interval, Note};
use contained_core::states::State;
use decanter::prelude::Hashable;
use petgraph::{Graph, Undirected};
use petgraph::graph::DefaultIx;
use serde::{Deserialize, Serialize};

pub type NoteGraph<E = Interval, Idx = DefaultIx> = Graph<Note, E, Undirected, Idx>;


#[derive(
    Clone,
    Debug,
    Default,
    Deserialize,
    Hashable,
    Serialize,
)]
pub struct TriadGraph {
    class: Triads,
    notes: NoteGraph<Interval, ChordFactor>,
    state: State,
}

impl std::fmt::Display for TriadGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}