/*
    Appellation: contained <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
#[cfg(feature = "core")]
pub use contained_core as core;
#[cfg(feature = "music")]
pub use contained_music as music;

pub use self::{primitives::*, utils::*};

mod primitives;
mod utils;

pub mod backend;
pub mod handle;
pub mod rt;

pub mod prelude {
    pub use super::backend::cli::*;
    pub use super::backend::*;
    pub use super::*;

    pub use super::rt::*;

    #[cfg(feature = "core")]
    pub use super::core::*;
    #[cfg(feature = "music")]
    pub use super::music::{classes::*, intervals::*, neo::*};
}
