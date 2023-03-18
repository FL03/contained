/*
    Appellation: surface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::Face;

pub struct Surface<V, W> {
    faces: Face<V>,
    edges: Vec<(V, V, W)>,
    vertices: Vec<V>
}