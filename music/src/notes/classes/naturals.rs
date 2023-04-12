/*
    Appellation: naturals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{Gradient, Pitch};

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
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum Naturals {
    #[default]
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
}

impl Gradient for Naturals {
    const MODULUS: i64 = 12;

    fn pitch(&self) -> i64 {
        *self as i64
    }
}

impl From<Naturals> for i64 {
    fn from(note: Naturals) -> i64 {
        note as i64
    }
}

impl TryFrom<Pitch> for Naturals {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Pitch) -> Result<Self, Self::Error> {
        Naturals::try_from(value.pitch())
    }
}

impl TryFrom<i64> for Naturals {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let data = value.abs() % 12;
        match data {
            0 => Ok(Self::C),
            2 => Ok(Self::D),
            4 => Ok(Self::E),
            5 => Ok(Self::F),
            7 => Ok(Self::G),
            9 => Ok(Self::A),
            11 => Ok(Self::B),
            _ => Err("".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_naturals() {
        assert!(Naturals::try_from(1).is_err());
        assert_eq!(Naturals::try_from(5).unwrap(), Naturals::F);
        assert_eq!(Naturals::from_str("a").unwrap(), Naturals::A);
    }
}
