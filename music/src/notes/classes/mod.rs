/*
    Appellation: classes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: the module is dedicated to implementing the various pitch classes
*/
pub use self::{accidentals::*, naturals::*};

pub(crate) mod accidentals;
pub(crate) mod naturals;

use crate::{Gradient, Pitch};
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
    const MODULUS: i64 = crate::MODULUS;

    fn class(&self) -> PitchClass {
        *self
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

impl<G: Gradient> From<&G> for PitchClass {
    fn from(value: &G) -> PitchClass {
        if let Ok(v) = Accidentals::try_from(value.pitch()) {
            PitchClass::from(v)
        } else {
            PitchClass::from(NaturalNote::try_from(value.pitch()).expect(""))
        }
    }
}

impl From<i64> for PitchClass {
    fn from(value: i64) -> PitchClass {
        PitchClass::from(&Pitch::new(value))
    }
}

impl From<PitchClass> for i64 {
    fn from(data: PitchClass) -> i64 {
        match data {
            PitchClass::Accidental(note) => note.into(),
            PitchClass::Natural(note) => note.into(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pitch_class() {
        let a = PitchClass::default();
        let b = PitchClass::Accidental(Default::default());
        assert_ne!(a.clone(), b.clone());
        assert_eq!(a, PitchClass::Natural(Default::default()));
        assert_eq!(
            b,
            PitchClass::Accidental(Accidentals::Sharp(Default::default()))
        )
    }
}