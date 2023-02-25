/*
    Appellation: Contained <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
#[cfg(feature = "actors")]
pub use contained_actors as actors;

pub use self::primitives::*;

pub(crate) mod primitives;

pub mod cmp;
pub mod neo;
