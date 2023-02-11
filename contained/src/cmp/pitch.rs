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
*/
use super::{Accidentals, NaturalNote};
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
        let data = value % 12;
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
        Self(pitch % 12)
    }
    pub fn pitch(&self) -> i64 {
        self.0
    }
    /// Simple way to detect if the pitch is natural or not
    pub fn is_natural(&self) -> bool {
        NaturalNote::try_from(self.pitch()).is_ok()
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
        assert_eq!(a, b);
        assert!(a.is_natural())
    }
}