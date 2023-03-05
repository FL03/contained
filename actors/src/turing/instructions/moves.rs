/*
    Appellation: moves <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        
*/
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

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
#[strum(serialize_all = "snake_case")]
pub enum Move {
    Left = 0,
    Right = 1,
    #[default]
    Stay = 2,
}

impl From<i64> for Move {
    fn from(d: i64) -> Self {
        match (d % 3).abs() {
            0 => Self::Left,
            1 => Self::Right,
            _ => Self::Stay,
        }
    }
}

impl From<Move> for i64 {
    fn from(d: Move) -> i64 {
        d as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_default() {
        let a = Move::default();
        assert_eq!(a.clone(), Move::Stay);
    }
}
