/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    EnumString,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    VariantNames,
)]
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum State {
    #[default]
    Valid = 0,
    Invalid = 1,
}

impl State {
    pub fn invalid() -> Self {
        Self::Invalid
    }
    pub fn valid() -> Self {
        Self::Valid
    }
    pub fn invalidate(&mut self) {
        *self = Self::Invalid;
    }
    pub fn validate(&mut self) {
        *self = Self::Valid;
    }
}

impl AsRef<[u8]> for State {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Invalid => b"invalid",
            Self::Valid => b"valid",
        }
    }
}

impl Unpin for State {}

impl std::ops::Mul for State {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Invalid => match rhs {
                Self::Invalid => Self::Invalid,
                Self::Valid => Self::Valid,
            },
            Self::Valid => match rhs {
                Self::Invalid => Self::Invalid,
                Self::Valid => Self::Valid,
            },
        }
    }
}

impl std::ops::MulAssign for State {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl From<i64> for State {
    fn from(d: i64) -> Self {
        Self::from(d.abs() as usize)
    }
}

impl From<usize> for State {
    fn from(d: usize) -> Self {
        match d % Self::COUNT {
            0 => State::valid(),
            _ => State::invalid(),
        }
    }
}

impl From<State> for i64 {
    fn from(d: State) -> i64 {
        d as i64
    }
}
