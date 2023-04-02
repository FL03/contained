/*
    Appellation: music <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{errors::*, notes::*, primitives::*, utils::*};

pub mod chords;
pub mod frequency;
pub mod intervals;
pub mod measure;
pub mod neo;
pub mod score;

mod errors;
mod notes;
mod primitives;
mod utils;

use intervals::Interval;
use std::ops::{AddAssign, SubAssign};

/// [Gradient] provides a numerical interpretation of a given object
pub trait Gradient:
    Clone + Eq + Ord + Into<i64> + AddAssign<Interval> + SubAssign<Interval>
{
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
