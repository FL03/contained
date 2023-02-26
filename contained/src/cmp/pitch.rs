/*
    Appellation: pitch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        A pitch essentially represents the frequency of a sound wave and has been mathematically expressed to be
            p = log(2f)
        empirically based on the octave doubling of frequency exponetially

        * All notes or pitches are of mod 12, giving us { 0, 1, ..., 10, 11 }
        * Sharp notes and flat notes are simply opposite; if sharp is up then flat is down
            For our purposes, sharp notes are represented with positive integers while flat notes are reserved for negatives

        Another possibility would be to describe natural notes as prime numbers as this would restrict their existance and remove any possible enharmonic pairings.
        More so, if we consider 1 to be a prime number
*/
use super::{Accidentals, Gradient, NaturalNote};
use crate::absmod;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
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
    SmartDefault,
)]
#[strum(serialize_all = "snake_case")]
pub enum PitchClass {
    Accidental(Accidentals),
    #[default]
    Natural(NaturalNote),
}

impl Gradient for PitchClass {
    fn pitch(&self) -> i64 {
        match *self {
            PitchClass::Accidental(v) => v.into(),
            PitchClass::Natural(v) => v.into(),
        }
    }
}

impl From<Accidentals> for PitchClass {
    fn from(data: Accidentals) -> PitchClass {
        PitchClass::Accidental(data)
    }
}

impl From<NaturalNote> for PitchClass {
    fn from(data: NaturalNote) -> PitchClass {
        PitchClass::Natural(data)
    }
}

impl From<PitchClass> for Pitch {
    fn from(data: PitchClass) -> Pitch {
        match data {
            PitchClass::Accidental(v) => v.into(),
            PitchClass::Natural(n) => Pitch::from(n as i64),
        }
    }
}

impl From<Pitch> for PitchClass {
    fn from(value: Pitch) -> PitchClass {
        PitchClass::from(value.pitch())
    }
}

impl From<i64> for PitchClass {
    fn from(value: i64) -> PitchClass {
        let data = Pitch::new(value);
        if let Ok(v) = Accidentals::try_from(data) {
            PitchClass::from(v)
        } else {
            PitchClass::from(NaturalNote::try_from(data).expect(""))
        }
    }
}

/// [Pitch] describes the modular index of a given frequency
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Pitch(i64);

impl Pitch {
    pub fn new(pitch: i64) -> Self {
        Self(pitch)
    }
    /// Simple way to detect if the pitch is natural or not
    pub fn is_natural(&self) -> bool {
        NaturalNote::try_from(self.pitch()).is_ok()
    }
}

impl Gradient for Pitch {
    fn pitch(&self) -> i64 {
        absmod(self.0, 12)
    }
}

impl std::ops::Add<i64> for Pitch {
    type Output = Pitch;

    fn add(self, rhs: i64) -> Self::Output {
        Pitch::new(self.0 + rhs)
    }
}

impl std::ops::Div<i64> for Pitch {
    type Output = Pitch;

    fn div(self, rhs: i64) -> Self::Output {
        Pitch::new(self.0 / rhs)
    }
}

impl std::ops::Mul<i64> for Pitch {
    type Output = Pitch;

    fn mul(self, rhs: i64) -> Self::Output {
        Pitch::new(self.0 * rhs)
    }
}

impl std::ops::Sub<i64> for Pitch {
    type Output = Pitch;

    fn sub(self, rhs: i64) -> Self::Output {
        Pitch::new(self.0 - rhs)
    }
}

impl std::fmt::Display for Pitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for Pitch {
    fn from(p: i64) -> Pitch {
        Pitch::new(p)
    }
}

impl From<Pitch> for i64 {
    fn from(p: Pitch) -> i64 {
        p.pitch()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmp::Accidentals;

    #[test]
    fn test_pitch_class() {
        let a = PitchClass::default();
        let b = PitchClass::Accidental(Accidentals::default());
        assert_ne!(a.clone(), b.clone());
        assert_eq!(a, PitchClass::Natural(Default::default()));
        assert_eq!(
            b,
            PitchClass::Accidental(Accidentals::Sharp(Default::default()))
        )
    }

    #[test]
    fn test_pitch() {
        let a = Pitch::from(144);
        let b = Pitch::from(12);
        assert_ne!(a, b);
        assert_eq!(a.pitch(), b.pitch());
        assert!(a.is_natural());
    }

    #[test]
    fn test_pitch_ops() {
        let pitch = Pitch::new(3);
        assert_eq!(pitch + 1, Pitch::new(4));
        assert_eq!(Pitch::new(5) * 2, Pitch::new(10));
    }
}
