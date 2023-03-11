/*
    Appellation: graphs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module is dedicated to graphs, explicitly implementing generic directed and undirected data-structures while providing the tools to create new ones.
*/
pub use self::{directed::*, undirected::*};

pub(crate) mod directed;
pub(crate) mod undirected;

pub mod cmp;

use cmp::{EdgeValue, GraphError, Node, Weight};
use std::collections::{HashMap, HashSet};

/// [Graph] describes the basic operations of a graph data-structure
pub trait Graph<N: Node = String, V: EdgeValue = Weight>: Clone {
    fn new() -> Self;
    /// [Graph::add_edge]
    fn add_edge(&mut self, edge: (N, N, V)) {
        self.add_node(edge.0.clone());
        self.add_node(edge.1.clone());

        self.adjacency_table_mut().entry(edge.0).and_modify(|e| {
            e.push((edge.1, edge.2));
        });
    }
    /// [Graph::add_node]
    fn add_node(&mut self, node: N) -> bool {
        match self.adjacency_table().get(&node) {
            None => {
                self.adjacency_table_mut().insert(node, Vec::new());
                true
            }
            _ => false,
        }
    }
    /// [Graph::adjacency_table_mut]
    fn adjacency_table_mut(&mut self) -> &mut HashMap<N, Vec<(N, V)>>;
    /// [Graph::adjacency_table]
    fn adjacency_table(&self) -> &HashMap<N, Vec<(N, V)>>;
    /// [Graph::contains]
    fn contains(&self, node: N) -> bool {
        self.adjacency_table().get(&node).is_some()
    }
    /// [Graph::edges]
    fn edges(&self) -> Vec<(N, N, V)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table().clone() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node.clone(), to_node, weight));
            }
        }
        edges
    }
    /// [Graph::neighbours]
    fn neighbours(&self, node: N) -> Result<&Vec<(N, V)>, GraphError> {
        match self.adjacency_table().get(&node) {
            None => Err(GraphError::NodeNotInGraph),
            Some(i) => Ok(i),
        }
    }
    /// [Graph::nodes]
    fn nodes(&self) -> HashSet<N> {
        self.adjacency_table().keys().cloned().collect()
    }
}
