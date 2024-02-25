/*
    Appellation: sevenths <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:

        Sevenths:
            Augmented (12)
            Major (11)
            Minor(10)
            Diminished (9)
*/
use crate::{BoxedError, Gradient, Note};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    EnumString,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
    VariantNames,
)]
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum Sevenths {
    Augmented = 12,
    Diminished = 9,
    #[default]
    Major = 11,
    Minor = 10,
}

impl Sevenths {
    pub fn new(from: Note, to: Note) -> Result<Self, BoxedError> {
        let interval = to.pitch() - from.pitch();
        match interval {
            12 => Ok(Sevenths::Augmented),
            9 => Ok(Sevenths::Diminished),
            11 => Ok(Sevenths::Major),
            10 => Ok(Sevenths::Minor),
            _ => Err("Invalid interval".into()),
        }
    }
    pub fn compute(&self, note: Note) -> Note {
        let interval = match self {
            Sevenths::Augmented => 12,
            Sevenths::Diminished => 9,
            Sevenths::Major => 11,
            Sevenths::Minor => 10,
        };
        let pitch = note.pitch() + interval;
        Note::from(pitch)
    }
}

impl std::ops::Add<Note> for Sevenths {
    type Output = Note;

    fn add(self, rhs: Note) -> Self::Output {
        (rhs.pitch() + self as i64).into()
    }
}

impl std::ops::Sub<Note> for Sevenths {
    type Output = Note;

    fn sub(self, rhs: Note) -> Self::Output {
        (rhs.pitch() - self as i64).into()
    }
}
