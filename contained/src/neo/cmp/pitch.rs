/*
    Appellation: pitch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::turing::Symbolic;
use serde::{Deserialize, Serialize};
use std::ops::Add;
use strum::{Display, EnumString, EnumVariantNames};

///
pub trait Pitch: Clone + Default + ToString {
    fn by_ref(self: &Self) -> &Self
    where
        Self: Sized,
    {
        self
    }
    fn by_mut_ref(self: &mut Self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }

    fn pitch(&self) -> &Self
    where
        Self: Sized,
    {
        &self
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum PitchClass {
    #[default]
    C = 0,
    #[strum(serialize = "c#")]
    Cs = 1,
    D = 2,
    #[strum(serialize = "d#")]
    Ds = 3,
    E = 4,
    F = 5,
    #[strum(serialize = "f#")]
    Fs = 6,
    G = 7,
    #[strum(serialize = "g#")]
    Gs = 8,
    A = 9,
    #[strum(serialize = "a#")]
    As = 10,
    B = 11,
}

impl Pitch for PitchClass {}

impl From<i64> for PitchClass {
    fn from(d: i64) -> PitchClass {
        let mut data = d;
        if data > 11 {
            data = data % 12;
        }
        match data {
            0 => Self::C,
            1 => Self::Cs,
            2 => Self::D,
            3 => Self::Ds,
            4 => Self::E,
            5 => Self::F,
            6 => Self::Fs,
            7 => Self::G,
            8 => Self::Gs,
            9 => Self::A,
            10 => Self::As,
            11 => Self::B,
            _ => Self::C,
        }
    }
}

impl Symbolic for PitchClass {}

impl From<PitchClass> for i64 {
    fn from(d: PitchClass) -> i64 {
        d as i64
    }
}

impl Add for PitchClass {
    type Output = String;

    fn add(self, rhs: Self) -> Self::Output {
        format!("{}{}", self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pitch_class() {
        let a = PitchClass::default();
        let b: PitchClass = 10.into();
        assert_eq!(a.to_string(), "c".to_string());
        assert_eq!(b.to_string(), "a#".to_string());
        assert_eq!(a + b, "ca#".to_string())
    }
}
