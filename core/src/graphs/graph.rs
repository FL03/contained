/*
    Appellation: graph <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::GraphData;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub trait Graph<T: GraphData> {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<T, Vec<(T, i32)>>;
    fn adjacency_table(&self) -> &HashMap<T, Vec<(T, i32)>>;

    fn add_node(&mut self, node: T) -> bool {
        match self.adjacency_table().get(&node) {
            None => {
                self.adjacency_table_mutable().insert(node, Vec::new());
                true
            }
            _ => false,
        }
    }

    fn add_edge(&mut self, edge: (T, T, i32)) {
        self.add_node(edge.0.clone());
        self.add_node(edge.1.clone());

        self.adjacency_table_mutable()
            .entry(edge.0)
            .and_modify(|e| {
                e.push((edge.1, edge.2));
            });
    }

    fn neighbours(&self, node: T) -> Result<&Vec<(T, i32)>, NodeNotInGraph> {
        match self.adjacency_table().get(&node) {
            None => Err(NodeNotInGraph),
            Some(i) => Ok(i),
        }
    }

    fn contains(&self, node: T) -> bool {
        self.adjacency_table().get(&node).is_some()
    }

    fn nodes(&self) -> HashSet<&T> {
        self.adjacency_table().keys().collect()
    }

    fn edges(&self) -> Vec<(&T, &T, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
