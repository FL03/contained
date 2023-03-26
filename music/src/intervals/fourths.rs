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
pub enum Fourths {
    #[default]
    Perfect = 5,
}

impl Fourths {
    pub fn new(from: Note, to: Note) -> Result<Self, BoxedError> {
        let interval = to.pitch() - from.pitch();
        match interval {
            5 => Ok(Fourths::Perfect),
            _ => Err("Invalid interval".into()),
        }
    }
}

impl Hashable for Fourths {
    fn hash(&self) -> H256 {
        hasher(self).into()
    }
}

impl TryFrom<(Note, Note)> for Fourths {
    type Error = BoxedError;

    fn try_from(data: (Note, Note)) -> Result<Self, Self::Error> {
        // An interval is the difference in pitch between an two notes
        // We take the pitch of the result to account for its modularity; (0, 11) -> 11 but (11, 0) -> 1
        let interval: i64 = (data.1.pitch() - data.0.pitch()).pitch();
        match interval {
            5 => Ok(Self::Perfect),
            _ => Err("Interval is not a fifth...".into()),
        }
    }
}

impl std::ops::Add<Note> for Fourths {
    type Output = Note;

    fn add(self, rhs: Note) -> Self::Output {
        (rhs.pitch() + self as i64).into()
    }
}

impl std::ops::Sub<Note> for Fourths {
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
    fn test_fourths() {
        assert_eq!(Fourths::Perfect + Note::from(0), Note::from(5))
    }
}
