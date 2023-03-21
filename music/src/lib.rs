/*
    Appellation: music <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{errors::*, notes::*, primitives::*, utils::*};

pub mod chord;
pub mod clef;
pub mod epoch;
pub mod frequency;
pub mod intervals;
pub mod neo;

pub(crate) mod errors;
pub(crate) mod notes;
pub(crate) mod primitives;
pub(crate) mod utils;

use crate::classes::{NaturalNote, PitchClass};
use algae::graph::cmp::Node;
use contained_core::Symbolic;

/// [Gradient] provides a numerical interpretation of a given object
pub trait Gradient: Clone + Eq + Ord + Into<i64> {
    const MODULUS: i64;

    fn class(&self) -> PitchClass {
        self.pitch().into()
    }
    /// [Gradient::pitch] is a method for numerically representing the structure
    fn pitch(&self) -> i64 {
        absmod(self.clone().into(), Self::MODULUS)
    }
}

impl Gradient for i64 {
    const MODULUS: i64 = 12;

    fn pitch(&self) -> i64 {
        absmod(*self, Self::MODULUS)
    }
}

/// [Notable] is used to designate a structure used to represent a note
pub trait Notable: From<i64> + Gradient + Symbolic + Node {
    /// [Notable::is_natural] Simple way to detect if the pitch is natural or not
    fn is_natural(&self) -> bool {
        NaturalNote::try_from(self.pitch()).is_ok()
    }
}
