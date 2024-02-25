/*
    Appellation: music <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Music
//!
//!
pub use self::{errors::*, notes::*, primitives::*, specs::*, utils::*};

pub mod chords;
pub mod frequency;
pub mod intervals;
pub mod neo;
pub mod score;

mod errors;
mod notes;
mod primitives;
mod specs;
mod utils;

pub mod prelude {
    pub use super::chords::*;
    pub use super::errors::*;
    pub use super::frequency::*;
    pub use super::intervals::*;
    pub use super::neo::{self, *};
    pub use super::notes::*;
    pub use super::primitives::*;
    pub use super::score::*;
    pub use super::specs::*;
    pub use super::utils::*;
}
