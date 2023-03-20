/*
    Appellation: surface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::Face;
use algae::graph::{cmp::Node, Graph, UndirectedGraph};

#[derive(Clone, Default)]
pub struct Surface<V: Node, W: Clone + PartialEq> {
    faces: Face<V>,
    graph: UndirectedGraph<V, W>
}

impl<V: Node, W: Clone + PartialEq> Surface<V, W> {
    
}
