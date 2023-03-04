/*
    Appellation: naturals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::music::{Gradient, Pitch};

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
pub enum NaturalNote {
    #[default]
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
}

impl From<NaturalNote> for i64 {
    fn from(note: NaturalNote) -> i64 {
        note as i64
    }
}

impl TryFrom<Pitch> for NaturalNote {
    type Error = String;

    fn try_from(value: Pitch) -> Result<Self, Self::Error> {
        NaturalNote::try_from(value.pitch())
    }
}

impl TryFrom<i64> for NaturalNote {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let data = value.clone() % 12;
        match data {
            0 => Ok(Self::C),
            2 => Ok(Self::D),
            4 => Ok(Self::E),
            5 => Ok(Self::F),
            7 => Ok(Self::G),
            9 => Ok(Self::A),
            11 => Ok(Self::B),
            _ => Err(format!("")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_naturals() {
        assert!(NaturalNote::try_from(1).is_err());
        assert_eq!(NaturalNote::try_from(5), Ok(NaturalNote::F));
        assert_eq!(NaturalNote::from_str("a"), Ok(NaturalNote::A))
    }
}
