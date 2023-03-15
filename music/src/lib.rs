/*
    Appellation: music <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{errors::*, notes::*, primitives::*, specs::*, utils::*};

pub mod chord;
pub mod clef;
pub mod epoch;
pub mod frequency;
pub mod intervals;
pub mod neo;

pub(crate) mod errors;
pub(crate) mod notes;
pub(crate) mod primitives;
pub(crate) mod specs;
pub(crate) mod utils;
