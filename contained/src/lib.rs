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

mod primitives;
mod utils;

pub mod agents;
pub mod backend;
pub mod connect;
pub mod vm;

pub mod prelude {
    pub use super::backend::cli::*;
    pub use super::backend::*;
    pub use super::*;

    #[cfg(feature = "core")]
    pub use super::core::*;
    #[cfg(feature = "music")]
    pub use super::music::{chords::*, intervals::*, neo::*};
    #[cfg(feature = "net")]
    pub use super::net::*;
}
