/*
    Appellation: reactions <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{State, Symbolic};
use scsys::prelude::fnl_remove;
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

pub struct MachineHead(State, String);

pub struct MachineTail(State, String, Move);
pub struct Instruction(MachineHead, MachineTail);


#[derive(Clone, Copy, Debug, Default, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, PartialOrd, Serialize)]
#[strum(serialize_all = "snake_case")]
pub enum Move {
    Left = 0,
    Right = 1,
    #[default]
    Stay = 2
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", fnl_remove(serde_json::to_string(self).unwrap()))
    }
}

impl From<i64> for Move {
    fn from(d: i64) -> Self {
        match d {
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