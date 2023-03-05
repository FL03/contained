/*
    Appellation: Contained <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
#[cfg(feature = "actors")]
pub use contained_actors as actors;
#[cfg(feature = "net")]
pub use contained_net as net;

pub use self::{primitives::*, specs::*, utils::*};

pub mod music;
pub mod neo;

pub(crate) mod primitives;
pub(crate) mod specs;
pub(crate) mod utils;
