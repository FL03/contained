/*
    Appellation: thirds <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A collection of common musical intervals
        A musical third can be either be a difference of three (minor) or four (major) semitones
*/
use crate::{BoxedError, Gradient, Note};
use decanter::prelude::Hashable;
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
    Hashable,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
)]
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum Thirds {
    #[default]
    Major = 4,
    Minor = 3,
}

impl Thirds {
    /// [is_third] compares two notes to see if either a major or minor third interval exists
    pub fn is_third(a: Note, b: Note) -> Result<Self, BoxedError> {
        Self::try_from((a, b))
    }
    pub fn compute(note: Note) -> (Note, Note) {
        (Self::Major + note, Self::Minor + note)
    }
    /// Functional method for creating a major third
    pub fn major() -> Self {
        Self::Major
    }
    /// Functional method for creating a minor third
    pub fn minor() -> Self {
        Self::Minor
    }
}

impl TryFrom<(Note, Note)> for Thirds {
    type Error = BoxedError;

    fn try_from(data: (Note, Note)) -> Result<Self, Self::Error> {
        // An interval is the difference in pitch between an two notes
        // We take the pitch of the result to account for its modularity; (0, 11) -> 11 but (11, 0) -> 1
        let interval: i64 = (data.1.pitch() - data.0.pitch()).pitch();
        match interval {
            3 => Ok(Self::Minor),
            4 => Ok(Self::Major),
            _ => Err("Interval is not a third...".into()),
        }
    }
}

impl TryFrom<[Note; 2]> for Thirds {
    type Error = BoxedError;

    fn try_from(data: [Note; 2]) -> Result<Self, Self::Error> {
        Thirds::try_from((data[0], data[1]))
    }
}

impl std::ops::Add<Note> for Thirds {
    type Output = Note;

    fn add(self, rhs: Note) -> Self::Output {
        (rhs.pitch() + self as i64).into()
    }
}

impl std::ops::Sub<Note> for Thirds {
    type Output = Note;

    fn sub(self, rhs: Note) -> Self::Output {
        (rhs.pitch() - self as i64).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Note;

    #[test]
    fn test_thirds() {
        assert_eq!(Thirds::Major + Note::from(0), Note::from(4))
    }
}
