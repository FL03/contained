/*
    Appellation: classes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: the module is dedicated to implementing the various pitch classes
*/
pub use self::{accidentals::*, naturals::*};

pub(crate) mod accidentals;
pub(crate) mod naturals;

use crate::{intervals::Interval, Gradient, Pitch};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

pub enum Classes {
    Flat,
    Sharp,
    Natural
}

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

impl PitchClass {
    pub fn new(value: i64) -> Self {
        if let Ok(v) = Accidentals::try_from(value) {
            PitchClass::from(v)
        } else {
            PitchClass::from(NaturalNote::try_from(value).expect(""))
        }
    }
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

impl std::ops::Add<Interval> for PitchClass {
    type Output = PitchClass;

    fn add(self, rhs: Interval) -> Self::Output {
        let interval: i64 = rhs.into();
        PitchClass::new(self.pitch() + interval)
    }
}

impl std::ops::AddAssign<Interval> for PitchClass {
    fn add_assign(&mut self, rhs: Interval) {
        *self = *self + rhs;
    }
}

impl std::ops::Div<Interval> for PitchClass {
    type Output = PitchClass;

    fn div(self, rhs: Interval) -> Self::Output {
        let interval: i64 = rhs.into();
        PitchClass::new(self.pitch() / interval)
    }
}

impl std::ops::DivAssign<Interval> for PitchClass {
    fn div_assign(&mut self, rhs: Interval) {
        *self = *self / rhs;
    }
}

impl std::ops::Mul<Interval> for PitchClass {
    type Output = PitchClass;

    fn mul(self, rhs: Interval) -> Self::Output {
        let interval: i64 = rhs.into();
        PitchClass::new(self.pitch() * interval)
    }
}

impl std::ops::MulAssign<Interval> for PitchClass {
    fn mul_assign(&mut self, rhs: Interval) {
        *self = *self * rhs;
    }
}

impl std::ops::Sub<Interval> for PitchClass {
    type Output = PitchClass;

    fn sub(self, rhs: Interval) -> Self::Output {
        let interval: i64 = rhs.into();
        PitchClass::new(self.pitch() - interval)
    }
}

impl std::ops::SubAssign<Interval> for PitchClass {
    fn sub_assign(&mut self, rhs: Interval) {
        *self = *self - rhs;
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
