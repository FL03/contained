/*
    Appellation: directed <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{Graph, GraphData};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DirectedGraph<T: GraphData> {
    adjacency_table: HashMap<T, Vec<(T, i32)>>,
}

impl<T: GraphData> Graph<T> for DirectedGraph<T> {
    fn new() -> Self {
        Self {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<T, Vec<(T, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<T, Vec<(T, i32)>> {
        &self.adjacency_table
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut graph = DirectedGraph::new();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert_eq!(graph.nodes(), [&"a", &"b", &"c"].iter().cloned().collect());
    }

    #[test]
    fn test_add_edge() {
        let mut graph = DirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("c", "a", 7));
        graph.add_edge(("b", "c", 10));

        let expected_edges = [(&"a", &"b", 5), (&"c", &"a", 7), (&"b", &"c", 10)];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }

    #[test]
    fn test_neighbours() {
        let mut graph = DirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        assert_eq!(graph.neighbours("a").unwrap(), &vec![("b", 5)]);
    }

    #[test]
    fn test_contains() {
        let mut graph = DirectedGraph::new();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert_eq!(graph.contains("a"), true);
        assert_eq!(graph.contains("b"), true);
        assert_eq!(graph.contains("c"), true);
        assert_eq!(graph.contains("d"), false);
    }
}
