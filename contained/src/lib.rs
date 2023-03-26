/*
    Appellation: contained <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
#[cfg(feature = "core")]
pub use contained_core as core;
#[cfg(feature = "music")]
pub use contained_music as music;

pub use self::{errors::*, primitives::*, utils::*};

mod errors;
mod primitives;
mod utils;

pub mod backend;

pub mod prelude {
    pub use super::*;
    
}