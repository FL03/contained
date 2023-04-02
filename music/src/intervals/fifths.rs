/*
    Appellation: fifths <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:

        Fifths:
            Augmented (8)
            Perfect (7)
            Diminished (6)
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
pub enum Fifths {
    Augmented = 8,
    Diminished = 6,
    #[default]
    Perfect = 7,
}

impl Fifths {
    pub fn compute(note: Note) -> (Note, Note, Note) {
        (
            Self::Augmented + note.clone(),
            Self::Perfect + note.clone(),
            Self::Diminished + note,
        )
    }
}

impl TryFrom<(Note, Note)> for Fifths {
    type Error = BoxedError;

    fn try_from(data: (Note, Note)) -> Result<Self, Self::Error> {
        // An interval is the difference in pitch between an two notes
        // We take the pitch of the result to account for its modularity; (0, 11) -> 11 but (11, 0) -> 1
        let interval: i64 = (data.1.pitch() - data.0.pitch()).pitch();
        match interval {
            6 => Ok(Self::Diminished),
            7 => Ok(Self::Perfect),
            8 => Ok(Self::Augmented),
            _ => Err("Interval is not a fifth...".into()),
        }
    }
}

impl std::ops::Add<Note> for Fifths {
    type Output = Note;

    fn add(self, rhs: Note) -> Self::Output {
        (rhs.pitch() + self as i64).into()
    }
}

impl std::ops::Sub<Note> for Fifths {
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
    fn test_fifths() {
        assert_eq!(Fifths::Perfect + Note::from(0), Note::from(7))
    }
}
