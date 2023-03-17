/*
    Appellation: contained <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
#[cfg(feature = "core")]
pub use contained_core as core;
#[cfg(feature = "music")]
pub use contained_music as music;
#[cfg(feature = "net")]
pub use contained_net as net;

pub use self::{primitives::*, utils::*};

pub(crate) mod primitives;
pub(crate) mod utils;

pub mod compute;
pub mod simplex;
