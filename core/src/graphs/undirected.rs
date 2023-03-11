/*
    Appellation: undirected <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{
    cmp::{EdgeValue, Node, Weight},
    Graph,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct UndirectedGraph<N: Node = String, V: EdgeValue = Weight> {
    adjacency_table: HashMap<N, Vec<(N, V)>>,
}

impl<N: Node, V: EdgeValue> Graph<N, V> for UndirectedGraph<N, V> {
    fn new() -> Self {
        Self {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mut(&mut self) -> &mut HashMap<N, Vec<(N, V)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<N, Vec<(N, V)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (N, N, V)) {
        self.add_node(edge.0.clone());
        self.add_node(edge.1.clone());

        self.adjacency_table.entry(edge.0.clone()).and_modify(|e| {
            e.push((edge.1.clone(), edge.2));
        });
        self.adjacency_table.entry(edge.1).and_modify(|e| {
            e.push((edge.0, edge.2));
        });
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;
    use super::UndirectedGraph;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            ("a", "b", 5),
            ("b", "a", 5),
            ("c", "a", 7),
            ("a", "c", 7),
            ("b", "c", 10),
            ("c", "b", 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }

    #[test]
    fn test_neighbours() {
        let mut graph = UndirectedGraph::new();

        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        assert_eq!(graph.neighbours("a").unwrap(), &vec![("b", 5), ("c", 7)]);
    }
}
