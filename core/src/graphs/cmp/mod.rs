/*
    Appellation: cmp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: components (cmp) for building effecient graph data-structures
*/
pub use self::{atable::*, errors::*};

pub(crate) mod atable;
pub(crate) mod errors;

/// [Weight] is a type alias describing the number system used by the [EdgeValue]
pub type Weight = i64;

/// [EdgeValue] is an optional value given to the edges of a [super::Graph] data-structure
/// Each value may have a symbolic representation, however, must be compatible with the [Weight]
pub trait EdgeValue: Clone + Copy + Into<Weight> + std::fmt::Debug + std::fmt::Display {}

impl EdgeValue for Weight {}

/// [Node] describes compatible vertices of the [super::Graph]
pub trait Node: Clone + Eq + std::hash::Hash {}

impl Node for char {}

impl Node for &str {}

impl Node for String {}
