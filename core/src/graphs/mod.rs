/*
    Appellation: torus <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        This module implements the data-structure behind the tonnetz
*/
pub use self::{directed::*, graph::*, undirected::*};

pub(crate) mod directed;
pub(crate) mod graph;
pub(crate) mod undirected;

pub trait GraphData: Clone + Eq + std::hash::Hash {}

impl GraphData for char {}

impl GraphData for &str {}

impl GraphData for String {}
