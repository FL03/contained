/*
    Appellation: edge <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: an edge consists of two nodes and an optional edge value
*/
use super::Node;

pub trait Related<N: Node, V> {

}



pub struct Edge<N: Node, V>(N, N, V);

impl<N: Node, V> Edge<N, V> {
    pub fn pair(self) -> (N, N) {
        (self.0, self.1)
    }
    pub fn weight(self) -> V {
        self.2
    }
}