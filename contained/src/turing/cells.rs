/*
    Appellation: cells <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use scsys::prelude::fnl_remove;
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum Cell {
    Zero = 0,
    One = 1,
    #[default]
    Blank = 2,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", fnl_remove(serde_json::to_string(self).unwrap()))
    }
}

impl From<i64> for Cell {
    fn from(d: i64) -> Self {
        match d {
            0 => Self::Zero,
            1 => Self::One,
            _ => Self::Blank,
        }
    }
}
impl From<Cell> for i64 {
    fn from(d: Cell) -> i64 {
        d as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_default() {
        let a = Cell::default();
        assert_eq!(a.clone(), Cell::Blank);
    }
}
