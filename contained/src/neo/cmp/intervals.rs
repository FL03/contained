/*
    Appellation: intervals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A collection of common musical intervals

        Thirds: Major / Minor
        Fifth: Augmented, Dimenished, Perfect
*/
use crate::neo::{cmp::Note, SEMITONE};
use serde::{Deserialize, Serialize};
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
    Thirds::Major.compute(pitch.into()).into()
}

///
pub fn minor_third(pitch: i64) -> i64 {
   Thirds::Minor.compute(pitch.into()).into()
}

///
pub fn perfect_fifth(pitch: i64) -> i64 {
    (pitch + 7) % 12
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
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
            Self::Minor => Note::from((n + 3 * SEMITONE as i64) % 12)
        }
    }
    pub fn major() -> Self {
        Self::Major
    }
    pub fn minor() -> Self {
        Self::Minor
    }
}
