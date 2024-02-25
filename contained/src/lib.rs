/*
    Appellation: contained <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Contained
///
/// A novel harmonic orchestration mechanism derived from the neo-Riemannian theory of music.
#[cfg(feature = "core")]
pub use contained_core as core;
#[cfg(feature = "music")]
pub use contained_music as music;
#[cfg(feature = "turing")]
pub use contained_turing as turing;

pub use self::{primitives::*, utils::*};

mod primitives;
mod utils;

pub mod agents;
pub mod cluster;

pub mod prelude {
    pub use super::primitives::*;
    pub use super::utils::*;

    #[cfg(feature = "core")]
    pub use super::core::prelude::*;
    #[cfg(feature = "music")]
    pub use super::music::prelude::*;
    #[cfg(feature = "turing")]
    pub use super::turing::prelude::*;
}
