/*
    Appellation: intervals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A collection of common musical intervals

        Thirds: Major / Minor
        Fifth: Augmented, Dimenished, Perfect
*/
use crate::{cmp::Note, SEMITONE};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

/// [is_third] compares two notes to see if either a major or minor third interval exists
pub fn is_third(a: i64, b: i64) -> bool {
    if is_major_third(a, b) || is_minor_third(a, b) {
        return true;
    }
    false
}

/// [is_major_third] compares the given notes and determines if a major third exists
pub fn is_major_third(a: i64, b: i64) -> bool {
    if major_third(a) == b {
        return true;
    }
    false
}

/// [is_minor_third]
pub fn is_minor_third(a: i64, b: i64) -> bool {
    if minor_third(a) == b {
        return true;
    }
    false
}

///
pub fn major_third(pitch: i64) -> i64 {
    let res = Thirds::Major * Note::from(pitch);
    res.into()
}

///
pub fn minor_third(pitch: i64) -> i64 {
    let res = Thirds::Minor * Note::from(pitch);
    res.into()
}

///
pub fn perfect_fifth(pitch: i64) -> i64 {
    let res = Fifths::Perfect * Note::from(pitch);
    res.into()
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
    Augmented = 1,
    Diminshed = 2,
    #[default]
    Perfect = 0,
}

impl Fifths {
    pub fn compute(&self, note: Note) -> Note {
        let pitch: i64 = note.into();
        match self {
            Fifths::Augmented => Note::from((pitch + 8) % 12),
            Fifths::Diminshed => Note::from((pitch + 6) % 12),
            Fifths::Perfect => Note::from((pitch + 7) % 12)
        }
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
    SmartDefault
)]
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum Thirds {
    #[default]
    Major = 0,
    Minor = 1,
}

impl Thirds {
    pub fn compute(&self, note: Note) -> Note {
        let n: i64 = note.into();
        match self {
            Self::Major => Note::from((n + 4 * SEMITONE as i64) % 12),
            Self::Minor => Note::from((n + 3 * SEMITONE as i64) % 12),
        }
    }
    pub fn major() -> Self {
        Self::Major
    }
    pub fn minor() -> Self {
        Self::Minor
    }
}

impl std::ops::Mul<Note> for Thirds {
    type Output = Note;

    fn mul(self, rhs: Note) -> Self::Output {
        self.compute(rhs)
    }
}
