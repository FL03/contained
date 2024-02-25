/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{absmod, Interval, PitchClass};
use std::ops::{AddAssign, SubAssign};

/// [IntervalMath] defines the operations that can be preformed with [Interval]s
pub trait IntervalMath: AddAssign<Interval> + SubAssign<Interval> {}

impl<T> IntervalMath for T where T: AddAssign<Interval> + SubAssign<Interval> {}

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

pub trait GradientExt: Gradient + IntervalMath {}

impl<T> GradientExt for T where T: Gradient + IntervalMath {}

impl Gradient for i64 {
    const MODULUS: i64 = 12;

    fn pitch(&self) -> i64 {
        absmod(*self, Self::MODULUS)
    }
}
