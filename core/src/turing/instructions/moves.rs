/*
    Appellation: moves <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:

*/
use crate::turing::Tape;
use crate::{turing::Symbolic, ArrayLike};
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
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum Move {
    Left = -1,
    Right = 1,
    #[default]
    Stay = 0,
}

impl Move {
    pub fn invert(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Stay => Self::Stay,
        }
    }
    pub fn apply<S: Symbolic>(
        &self,
        mut index: usize,
        mut tape: Tape<S>,
        elem: S,
    ) -> (usize, Tape<S>) {
        match *self {
            // If the current position is 0, insert a new element at the top of the vector
            Move::Left if index == 0 => {
                tape[index] = elem;
            }
            Move::Left => {
                index -= 1;
            }
            Move::Right => {
                index += 1;

                if index == tape.len() {
                    tape[index] = elem;
                }
            }
            Move::Stay => {}
        };
        (index, tape.clone())
    }
    pub fn shift(&self, pos: usize) -> usize {
        (pos as i64 + *self as i64) as usize
    }
}

impl std::ops::Mul<Move> for usize {
    type Output = usize;

    fn mul(self, rhs: Move) -> Self::Output {
        rhs.shift(self)
    }
}

impl From<i64> for Move {
    fn from(d: i64) -> Self {
        match d % 2 {
            -1 => Self::Left,
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
