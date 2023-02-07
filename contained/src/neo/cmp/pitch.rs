/*
    Appellation: pitch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        A pitch essentially represents the frequency of a sound wave and has been mathematically expressed to be
            p = log(2f)
        empirically based on the octave doubling of frequency exponetially
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
    A = 0,
    #[strum(serialize = "a#")]
    As = 1,
    B = 2,
    #[default]
    C = 3,
    #[strum(serialize = "c#")]
    Cs = 4,
    D = 5,
    #[strum(serialize = "d#")]
    Ds = 6,
    E = 7,
    F = 8,
    #[strum(serialize = "f#")]
    Fs = 9,
    G = 10,
    #[strum(serialize = "g#")]
    Gs = 11,
}

impl Pitch for PitchClass {}

impl From<i64> for PitchClass {
    fn from(d: i64) -> PitchClass {
        let mut data = d;
        if data > 11 {
            data = data % 12;
        }
        match data {
            0 => Self::A,
            1 => Self::As,
            2 => Self::B,
            3 => Self::C,
            4 => Self::Cs,
            5 => Self::D,
            6 => Self::Ds,
            7 => Self::E,
            8 => Self::F,
            9 => Self::Fs,
            10 => Self::G,
            _ => Self::Gs,
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

    #[test]
    fn test_modularity() {
        let a = PitchClass::from(144);
        assert_eq!(a.clone(), PitchClass::C);
    }
}
