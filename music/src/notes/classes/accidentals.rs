/*
    Appellation: accidentals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Accidental notes are either sharp or flat
*/
use super::Naturals;
use crate::Pitch;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumIter, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
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
#[strum(serialize_all = "snake_case")]
pub enum Accidentals {
    Flat(FlatNote),
    #[default]
    Sharp(SharpNote),
}

impl From<Accidentals> for i64 {
    fn from(note: Accidentals) -> i64 {
        match note {
            Accidentals::Sharp(s) => s.into(),
            Accidentals::Flat(f) => f.into(),
        }
    }
}

impl TryFrom<Pitch> for Accidentals {
    type Error = Box<dyn std::error::Error>;

    fn try_from(data: Pitch) -> Result<Accidentals, Self::Error> {
        if Naturals::try_from(data).is_err() {
            let note = if data.pitch() >= 0 {
                Accidentals::Sharp(SharpNote::try_from(data)?)
            } else {
                Accidentals::Flat(FlatNote::try_from(data)?)
            };
            return Ok(note);
        }
        Err("Provided note is natural".into())
    }
}

impl TryFrom<i64> for Accidentals {
    type Error = Box<dyn std::error::Error>;

    fn try_from(data: i64) -> Result<Accidentals, Self::Error> {
        Accidentals::try_from(Pitch::new(data))
    }
}

impl From<Accidentals> for Pitch {
    fn from(data: Accidentals) -> Pitch {
        let pitch = match data {
            Accidentals::Flat(n) => n as i64,
            Accidentals::Sharp(n) => n as i64,
        };
        Pitch::from(pitch)
    }
}

impl From<FlatNote> for Accidentals {
    fn from(data: FlatNote) -> Accidentals {
        Accidentals::Flat(data)
    }
}

impl From<SharpNote> for Accidentals {
    fn from(data: SharpNote) -> Accidentals {
        Accidentals::Sharp(data)
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
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
pub enum FlatNote {
    A = 8,
    B = 10,
    #[default]
    D = 1,
    E = 3,
    G = 6,
}

impl From<FlatNote> for i64 {
    fn from(note: FlatNote) -> i64 {
        note as i64
    }
}

impl TryFrom<i64> for FlatNote {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let data = value.abs() % 12;
        match data {
            1 => Ok(Self::D),
            3 => Ok(Self::E),
            6 => Ok(Self::G),
            8 => Ok(Self::A),
            10 => Ok(Self::B),
            _ => Err("".into()),
        }
    }
}

impl TryFrom<Pitch> for FlatNote {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Pitch) -> Result<Self, Self::Error> {
        FlatNote::try_from(value.pitch())
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumIter,
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
pub enum SharpNote {
    A = 10,
    #[default]
    C = 1,
    D = 3,
    F = 6,
    G = 8,
}

impl From<SharpNote> for i64 {
    fn from(note: SharpNote) -> i64 {
        note as i64
    }
}

impl TryFrom<Pitch> for SharpNote {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Pitch) -> Result<Self, Self::Error> {
        SharpNote::try_from(value.pitch())
    }
}

impl TryFrom<i64> for SharpNote {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let data = value % 12;
        match data {
            1 => Ok(Self::C),
            3 => Ok(Self::D),
            6 => Ok(Self::F),
            8 => Ok(Self::G),
            10 => Ok(Self::A),
            _ => Err("".into()),
        }
    }
}
