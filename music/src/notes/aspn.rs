/*
    Appellation: note <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: aspn is short for american scientific pitch notation; it is a specific type of note which denotes a certain octave for the given pitch-class
*/
use super::PitchClass;
use crate::{intervals::Interval, Gradient, MODULUS};
use serde::{Deserialize, Serialize};

/// [ASPN] is a specific type of [Note] which denotes a certain octave for the given pitch-class
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct ASPN {
    class: PitchClass,
    octave: i64,
}

impl ASPN {
    pub fn new(class: PitchClass, octave: Option<i64>) -> Self {
        Self {
            class,
            octave: octave.unwrap_or(1),
        }
    }
}

impl std::fmt::Display for ASPN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.class, self.octave)
    }
}

impl Gradient for ASPN {
    const MODULUS: i64 = MODULUS;

    fn class(&self) -> PitchClass {
        self.class
    }
    fn pitch(&self) -> i64 {
        self.class.into()
    }
}

impl std::ops::AddAssign<Interval> for ASPN {
    fn add_assign(&mut self, rhs: Interval) {
        // self.octave += interval / Self::MODULUS;
        self.class += rhs;
    }
}

impl std::ops::SubAssign<Interval> for ASPN {
    fn sub_assign(&mut self, rhs: Interval) {
        // self.octave += interval / Self::MODULUS;
        self.class -= rhs;
    }
}

impl From<ASPN> for i64 {
    fn from(data: ASPN) -> i64 {
        data.pitch()
    }
}
