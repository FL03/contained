/*
    Appellation: cmp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A collection of musical objects abstracted for computational purposes
*/
pub use self::{chord::*, clef::*, epoch::*, intervals::*, notes::*, pitch::*};

pub(crate) mod chord;
pub(crate) mod clef;
pub(crate) mod epoch;
pub(crate) mod intervals;
pub(crate) mod notes;
pub(crate) mod pitch;

pub trait Gradient: Clone + std::convert::Into<i64> {
    fn class(&self) -> PitchClass {
        PitchClass::from(self.pitch())
    }
    fn pitch(&self) -> i64 {
        crate::absmod(self.clone().into(), 12)
    }
}

impl Gradient for i64 {
    fn pitch(&self) -> i64 {
        // Adding twelve to the number accounts for negative modulo
        // For example, if self is -1 than adding 12 gives us a result of 11.
        (((self % 12) + 12) % 12).abs()
    }
}
