/*
    Appellation: clef <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        A clef, placed on the far left-hand side of the staff or stave, signals which notes are represented by the respective staff.
*/
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, VariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumString,
    Eq,
    Hash,
    PartialEq,
    PartialOrd,
    Serialize,
    VariantNames
)]
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum Clef {
    Alto = 0,
    Bass = 1,
    #[default]
    Treble = 2,
}

impl From<Clef> for i64 {
    fn from(data: Clef) -> i64 {
        data as i64
    }
}

impl From<i64> for Clef {
    fn from(data: i64) -> Clef {
        match data {
            0 => Clef::Alto,
            1 => Clef::Bass,
            _ => Clef::Treble,
        }
    }
}
