/*
    Appellation: cmp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: components (cmp) for building effecient graph data-structures
*/
pub use self::{atable::*, edge::*, errors::*};

pub(crate) mod atable;
pub(crate) mod edge;
pub(crate) mod errors;


/// [Node] describes compatible vertices of the [super::Graph]
pub trait Node: Clone + Eq + std::hash::Hash {}

impl Node for char {}

impl Node for &str {}

impl Node for String {}
