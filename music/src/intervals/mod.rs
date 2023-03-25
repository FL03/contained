/*
    Appellation: intervals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A collection of common musical intervals
*/
pub use self::{fifths::*, sevenths::*, thirds::*};

pub(crate) mod fifths;
pub(crate) mod sevenths;
pub(crate) mod thirds;

use crate::{Gradient, Note};

use decanter::prelude::{hasher, Hashable, H256};
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
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum Interval {
    Semitone = 1,
    Tone = 2,
    #[default]
    Third(Thirds),
    Fifth(Fifths),
    Seventh(Sevenths),
    Interval
}

impl Interval {
    pub fn new(from: Note, to: Note) -> Self {
        let interval = to.pitch() - from.pitch();
        match interval {
            1 => Interval::Semitone,
            2 => Interval::Tone,
            3 => Interval::Third(Thirds::Minor),
            4 => Interval::Third(Thirds::Major),
            6 => Interval::Fifth(Fifths::Diminished),
            7 => Interval::Fifth(Fifths::Perfect),
            8 => Interval::Fifth(Fifths::Augmented),
            9 => Interval::Seventh(Sevenths::Diminished),
            10 => Interval::Seventh(Sevenths::Major),
            11 => Interval::Seventh(Sevenths::Minor),
            12 => Interval::Seventh(Sevenths::Augmented),
            _ => Interval::Interval,
        }
    }
    pub fn increase(&self, note: Note) -> Note {
        let interval: i64 = self.clone().into();
        (note.pitch() + interval).into()
    }
    pub fn decrease(&self, note: Note) -> Note {
        let interval: i64 = self.clone().into();
        (note.pitch() - interval).into()
    }
}

impl Hashable for Interval {
    fn hash(&self) -> H256 {
        hasher(self).into()
    }
}

impl From<Interval> for i64 {
    fn from(interval: Interval) -> i64 {
        match interval {
            Interval::Semitone => 1,
            Interval::Tone => 2,
            Interval::Third(i) => i as i64,
            Interval::Fifth(i) => i as i64,
            Interval::Seventh(i) => i as i64,
            Interval::Interval => 5,
        }
    }
}

impl From<Fifths> for Interval {
    fn from(data: Fifths) -> Interval {
        Interval::Fifth(data)
    }
}

impl From<Thirds> for Interval {
    fn from(data: Thirds) -> Interval {
        Interval::Third(data)
    }
}

impl std::ops::Add<Interval> for i64 {
    type Output = i64;

    fn add(self, rhs: Interval) -> Self::Output {
        let interval: i64 = rhs.into();
        (self + interval).pitch()
    }
}

impl std::ops::AddAssign<Interval> for i64 {
    fn add_assign(&mut self, rhs: Interval) {
        let interval: i64 = rhs.into();
        *self = (*self + interval).pitch();
    }
}

impl std::ops::Sub<Interval> for i64 {
    type Output = i64;

    fn sub(self, rhs: Interval) -> Self::Output {
        let interval: i64 = rhs.into();
        (self - interval).pitch()
    }
}

impl std::ops::SubAssign<Interval> for i64 {
    fn sub_assign(&mut self, rhs: Interval) {
        let interval: i64 = rhs.into();
        *self = (*self - interval).pitch();
    }
}

impl std::ops::Add<Note> for Interval {
    type Output = Note;

    fn add(self, rhs: Note) -> Self::Output {
        let interval: i64 = self.into();
        (rhs.pitch() + interval).into()
    }
}

impl std::ops::Sub<Note> for Interval {
    type Output = Note;

    fn sub(self, rhs: Note) -> Self::Output {
        let interval: i64 = self.into();
        (rhs.pitch() - interval).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Note;

    #[test]
    fn test_interval() {
        assert_eq!(Interval::from(Thirds::Major) + Note::from(0), Note::from(4))
    }
}
