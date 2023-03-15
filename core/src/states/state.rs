/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::StateSpec;
use decanter::prelude::{hasher, Hashable, H256};
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
}

impl Hashable for State {
    fn hash(&self) -> H256 {
        hasher(self).into()
    }
}

impl StateSpec for State {}

impl std::ops::Add for State {
    type Output = State;

    fn add(self, rhs: Self) -> Self::Output {
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

impl std::ops::AddAssign for State {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl From<usize> for State {
    fn from(d: usize) -> Self {
        Self::from(d as i64)
    }
}

impl From<i64> for State {
    fn from(d: i64) -> Self {
        match d.abs() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state() {
        let a = State::default();
        let mut b = a;
        b += a;
        assert_eq!(a, State::valid());
        assert_eq!(b, State::valid());
    }
}
