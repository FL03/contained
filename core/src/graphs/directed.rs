/*
    Appellation: directed <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{cmp::{AdjacencyTable, Node}, Graph};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DirectedGraph<N: Node = String, V: Clone = i64> {
    adjacency_table: AdjacencyTable<N, V>,
}

impl<N: Node, V: Clone> Graph<N, V> for DirectedGraph<N, V> {
    fn new() -> Self {
        Self {
            adjacency_table: AdjacencyTable::new(),
        }
    }
    fn adjacency_table_mut(&mut self) -> &mut AdjacencyTable<N, V> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &AdjacencyTable<N, V> {
        &self.adjacency_table
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut graph = DirectedGraph::<&str, i64>::new();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert_eq!(graph.nodes(), ["a", "b", "c"].iter().cloned().collect());
    }

    #[test]
    fn test_add_edge() {
        let mut graph = DirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("c", "a", 7));
        graph.add_edge(("b", "c", 10));

        let expected_edges = [("a", "b", 5), ("c", "a", 7), ("b", "c", 10)];
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
        let mut graph = DirectedGraph::<&str, i64>::new();
        graph.add_node("a");
        graph.add_node("b");
        graph.add_node("c");
        assert_eq!(graph.contains("a"), true);
        assert_eq!(graph.contains("b"), true);
        assert_eq!(graph.contains("c"), true);
        assert_eq!(graph.contains("d"), false);
    }
}
