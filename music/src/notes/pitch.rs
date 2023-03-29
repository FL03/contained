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
use super::Note;
use crate::{intervals::Interval, Gradient};
use serde::{Deserialize, Serialize};

/// [Pitch] describes the modular index of a given frequency
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Pitch(i64);

impl Pitch {
    pub fn new(pitch: i64) -> Self {
        Self(pitch)
    }
}

impl Gradient for Pitch {
    const MODULUS: i64 = crate::MODULUS;

    fn pitch(&self) -> i64 {
        crate::absmod(self.0, Self::MODULUS)
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

impl From<Note> for Pitch {
    fn from(note: Note) -> Pitch {
        Pitch::new(note.pitch())
    }
}

impl From<Pitch> for i64 {
    fn from(p: Pitch) -> i64 {
        p.0
    }
}

impl std::ops::Add<Interval> for Pitch {
    type Output = Pitch;

    fn add(self, rhs: Interval) -> Self::Output {
        let interval: i64 = rhs.into();
        Pitch::new(self.0 + interval)
    }
}

impl std::ops::AddAssign<Interval> for Pitch {
    fn add_assign(&mut self, rhs: Interval) {
        *self = *self + rhs;
    }
}

impl std::ops::Div<Interval> for Pitch {
    type Output = Pitch;

    fn div(self, rhs: Interval) -> Self::Output {
        let interval: i64 = rhs.into();
        Pitch::new(self.0 / interval)
    }
}

impl std::ops::DivAssign<Interval> for Pitch {
    fn div_assign(&mut self, rhs: Interval) {
        *self = *self / rhs;
    }
}

impl std::ops::Mul<Interval> for Pitch {
    type Output = Pitch;

    fn mul(self, rhs: Interval) -> Self::Output {
        let interval: i64 = rhs.into();
        Pitch::new(self.0 * interval)
    }
}

impl std::ops::MulAssign<Interval> for Pitch {
    fn mul_assign(&mut self, rhs: Interval) {
        *self = *self * rhs;
    }
}

impl std::ops::Sub<Interval> for Pitch {
    type Output = Pitch;

    fn sub(self, rhs: Interval) -> Self::Output {
        let interval: i64 = rhs.into();
        Pitch::new(self.0 - interval)
    }
}

impl std::ops::SubAssign<Interval> for Pitch {
    fn sub_assign(&mut self, rhs: Interval) {
        *self = *self - rhs;
    }
}

impl<P: Gradient> std::ops::Add<P> for Pitch {
    type Output = Pitch;

    fn add(self, rhs: P) -> Self::Output {
        Pitch::new(self.0 + rhs.pitch())
    }
}

impl<P: Gradient> std::ops::AddAssign<P> for Pitch {
    fn add_assign(&mut self, rhs: P) {
        *self = *self + rhs.pitch();
    }
}

impl<P: Gradient> std::ops::Div<P> for Pitch {
    type Output = Pitch;

    fn div(self, rhs: P) -> Self::Output {
        Pitch::new(self.0 / rhs.pitch())
    }
}

impl<P: Gradient> std::ops::DivAssign<P> for Pitch {
    fn div_assign(&mut self, rhs: P) {
        *self = *self / rhs.pitch();
    }
}

impl<P: Gradient> std::ops::Mul<P> for Pitch {
    type Output = Pitch;

    fn mul(self, rhs: P) -> Self::Output {
        Pitch::new(self.0 * rhs.pitch())
    }
}

impl<P: Gradient> std::ops::MulAssign<P> for Pitch {
    fn mul_assign(&mut self, rhs: P) {
        *self = *self * rhs.pitch();
    }
}

impl<P: Gradient> std::ops::Sub<P> for Pitch {
    type Output = Pitch;

    fn sub(self, rhs: P) -> Self::Output {
        Pitch::new(self.0 - rhs.pitch())
    }
}

impl<P: Gradient> std::ops::SubAssign<P> for Pitch {
    fn sub_assign(&mut self, rhs: P) {
        *self = self.clone() - rhs.pitch();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Accidentals, Gradient, PitchClass};

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

    #[test]
    fn test_pitch() {
        let a = Pitch::from(144);
        let b = Pitch::from(12);
        assert_ne!(a, b);
        assert_eq!(a.pitch(), b.pitch());
    }

    #[test]
    fn test_pitch_ops() {
        let pitch = Pitch::new(3);
        assert_eq!(pitch + 1, Pitch::new(4));
        assert_eq!(Pitch::new(5) * 2, Pitch::new(10));
    }
}
