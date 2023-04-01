/*
    Appellation: note <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        A note is a symbolic representation of the duration and pitch of a tone;
        The enharmonic nature of musical notation enables us to create a system entirely
        dependent upon the modulus of the Pitch rather than the specific symbol.
        That being said, we will also adopt a note representation similar to that of the
        American Scientific Pitch Notation which denotes a certain octave for the given pitch-class.
*/
use super::{Pitch, PitchClass, ASPN};
use crate::{intervals::Interval, Gradient};
use algae::graph::cmp::Node;
use contained_core::turing::Symbolic;
use serde::{Deserialize, Serialize};

/// A [Note] is simply a wrapper for a [PitchClass], providing additional information such as an octave ([i64])
/// This type of musical notation is adopted from the American Scientific Pitch Notation
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Note {
    class: PitchClass,
    octave: i64,
}

impl Note {
    pub fn new(class: PitchClass, octave: Option<i64>) -> Self {
        Self {
            class,
            octave: octave.unwrap_or(1),
        }
    }
    pub fn aspn(&self) -> ASPN {
        ASPN::new(self.class, Some(self.octave))
    }
    pub fn interval(&self, other: &Self) -> Interval {
        Interval::new(self.clone(), other.clone())
    }
    pub fn octave(&self) -> i64 {
        self.octave
    }
}

impl Gradient for Note {
    const MODULUS: i64 = crate::MODULUS;

    fn class(&self) -> PitchClass {
        self.class
    }
    fn pitch(&self) -> i64 {
        self.class.into()
    }
}

impl Node for Note {}

impl Symbolic for Note {}

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.class, self.octave)
    }
}

impl<P: Gradient> std::ops::Add<P> for Note {
    type Output = Self;

    fn add(self, rhs: P) -> Self::Output {
        (self.pitch() + rhs.pitch()).into()
    }
}

impl<P: Gradient> std::ops::AddAssign<P> for Note {
    fn add_assign(&mut self, rhs: P) {
        *self = self.clone() + rhs;
    }
}

impl<P: Gradient> std::ops::Sub<P> for Note {
    type Output = Self;

    fn sub(self, rhs: P) -> Self::Output {
        (self.pitch() - rhs.pitch()).into()
    }
}

impl<P: Gradient> std::ops::SubAssign<P> for Note {
    fn sub_assign(&mut self, rhs: P) {
        *self = self.clone() - rhs;
    }
}

impl std::ops::Add<Interval> for Note {
    type Output = Self;

    fn add(self, rhs: Interval) -> Self::Output {
        let interval: i64 = rhs.into();
        (self.pitch() + interval).into()
    }
}

impl std::ops::AddAssign<Interval> for Note {
    fn add_assign(&mut self, rhs: Interval) {
        let interval: i64 = rhs.into();
        *self = (self.pitch() + interval).into()
    }
}

impl std::ops::Sub<Interval> for Note {
    type Output = Self;

    fn sub(self, rhs: Interval) -> Self::Output {
        let interval: i64 = rhs.into();
        (self.pitch() - interval).into()
    }
}

impl std::ops::SubAssign<Interval> for Note {
    fn sub_assign(&mut self, rhs: Interval) {
        let interval: i64 = rhs.into();
        *self = (self.pitch() - interval).into()
    }
}

impl From<i64> for Note {
    fn from(data: i64) -> Note {
        Note::new(PitchClass::from(&Pitch::new(data)), None)
    }
}

impl From<Pitch> for Note {
    fn from(data: Pitch) -> Note {
        Note::new(PitchClass::from(&data), None)
    }
}

impl<P: Gradient> From<&P> for Note {
    fn from(d: &P) -> Note {
        Note::new(d.class(), None)
    }
}

impl From<Note> for i64 {
    fn from(data: Note) -> i64 {
        data.class.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NaturalNote;

    #[test]
    fn test_notes() {
        let a = Note::new(PitchClass::Natural(NaturalNote::C), None);
        assert_eq!(a.pitch(), 0);
        assert_eq!(a.octave(), 1);
    }
}
