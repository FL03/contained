/*
    Appellation: surface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        A surface is used to describe a topological object that generally extends a graph data-structure by adding an additional surface value.
        The surface value or area typically describes all of the possible results of a transition function and is used to establish a consistent state
*/

pub trait Surface {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn edges(&self) -> u32;
    fn vertices(&self) -> u32;
    fn faces(&self) -> u32;
    fn volume(&self) -> f64;
}

pub trait Polytope {
    fn dim(&self) -> u32;
}
