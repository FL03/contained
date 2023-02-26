/*
    Appellation: intervals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A collection of common musical intervals

        Thirds: Major / Minor
        Fifth: Augmented, Dimenished, Perfect
*/
use super::{Gradient, Note, Pitch};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

/// [is_third] compares two notes to see if either a major or minor third interval exists
pub fn is_third(a: impl Gradient, b: impl Gradient) -> bool {
    if Thirds::Major * a.pitch() == b.pitch() || Thirds::Minor * a.pitch() == b.pitch() {
        return true;
    }
    false
}

/// [is_major_third] compares the given notes and determines if a major third exists
pub fn is_major_third(a: impl Gradient, b: impl Gradient) -> bool {
    if Thirds::Major * a.pitch() == b.pitch() {
        return true;
    }
    false
}

/// [is_minor_third]
pub fn is_minor_third(a: impl Gradient, b: impl Gradient) -> bool {
    if Thirds::Minor * a.pitch() == b.pitch() {
        return true;
    }
    false
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
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum Interval {
    Fifth(Fifths),
    #[default]
    Third(Thirds),
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
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum Fifths {
    Augmented = 8,
    Diminshed = 6,
    #[default]
    Perfect = 7,
}

impl Fifths {
    pub fn compute(&self, note: impl Gradient) -> Note {
        Note::from(note.pitch() + *self as i64)
    }
}

impl std::ops::Mul<i64> for Fifths {
    type Output = i64;

    fn mul(self, rhs: i64) -> Self::Output {
        self.compute(rhs).into()
    }
}

impl std::ops::Mul<Pitch> for Fifths {
    type Output = Pitch;

    fn mul(self, rhs: Pitch) -> Self::Output {
        self.compute(rhs).into()
    }
}

impl std::ops::Mul<Note> for Fifths {
    type Output = Note;

    fn mul(self, rhs: Note) -> Self::Output {
        self.compute(rhs)
    }
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
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum Thirds {
    #[default]
    Major = 4,
    Minor = 3,
}

impl Thirds {
    pub fn compute(&self, note: impl Gradient) -> Note {
        Note::from(note.pitch() + *self as i64)
    }
    pub fn compute_both(note: impl Gradient + Clone) -> (Note, Note) {
        (Self::Major.compute(note.clone()), Self::Minor.compute(note))
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

impl std::ops::Mul<i64> for Thirds {
    type Output = i64;

    fn mul(self, rhs: i64) -> Self::Output {
        self.compute(rhs).into()
    }
}

impl std::ops::Mul<Pitch> for Thirds {
    type Output = Pitch;

    fn mul(self, rhs: Pitch) -> Self::Output {
        self.compute(rhs).into()
    }
}

impl std::ops::Mul<Note> for Thirds {
    type Output = Note;

    fn mul(self, rhs: Note) -> Self::Output {
        self.compute(rhs)
    }
}
