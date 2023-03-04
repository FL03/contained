/*
    Appellation: cmp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A collection of musical objects abstracted for computational purposes
*/
pub use self::{notes::*, pitch::*};

pub mod chord;
pub mod clef;
pub mod epoch;
pub mod intervals;

pub(crate) mod notes;
pub(crate) mod pitch;

use crate::absmod;
pub trait Gradient: Clone + std::convert::Into<i64> {
    fn class(&self) -> PitchClass {
        PitchClass::from(&self.pitch())
    }
    /// [Gradient::pitch] is a method for numerically representing the structure
    fn pitch(&self) -> i64 {
        absmod(self.clone().into(), 12)
    }
}

impl Gradient for i64 {}

/// [Notable] is used to designate a structure used to represent a note
pub trait Notable:
    Clone + Default + Gradient + PartialEq + Send + Sync + ToString + std::convert::From<i64>
{
    /// [Notable::is_natural] Simple way to detect if the pitch is natural or not
    fn is_natural(&self) -> bool {
        NaturalNote::try_from(self.pitch()).is_ok()
    }
}
