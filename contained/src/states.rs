/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use decanter::prelude::{hasher, Hashable};
use scsys::prelude::{SerdeDisplay, StatePack};
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone,
    Debug,
    Default,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    SerdeDisplay,
    Serialize,
)]
pub struct State {
    pub message: String,
    state: States,
}

impl State {
    pub fn new(message: String, state: States) -> Self {
        Self { message, state }
    }
    pub fn state(&self) -> States {
        self.state
    }
    pub fn update_state(&mut self, state: States) {
        self.state = state;

    }
}

impl Hashable for State {
    fn hash(&self) -> decanter::prelude::H256 {
        hasher(&self).into()
    }
}

impl From<i64> for State {
    fn from(data: i64) -> State {
        Self::new(String::new(), data.into())
    }
}

impl From<State> for i64 {
    fn from(data: State) -> i64 {
        data.state.into()
    }
}

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
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum States {
    Invalid = 0,
    #[default]
    Valid = 1,
}

impl States {
    pub fn invalid() -> Self {
        Self::Invalid
    }
    pub fn valid() -> Self {
        Self::Valid
    }
}

impl Hashable for States {
    fn hash(&self) -> decanter::prelude::H256 {
        hasher(&self).into()
    }
}

impl StatePack for States {}

impl std::fmt::Display for States {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = match self.clone() as i64 {
            0 => "invalid",
            _ => "valid",
        };
        write!(f, "{}", p)
    }
}

impl From<i64> for States {
    fn from(val: i64) -> States {
        match val {
            0 => States::Invalid,
            _ => States::Valid,
        }
    }
}

impl From<States> for i64 {
    fn from(val: States) -> i64 {
        val as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_default_state() {
        let a = State::default();
        let mut b = a.clone();

        assert_eq!(&a, &b);
        assert_eq!(a.state() as i64, 1);

        b.update_state(10.into());
        assert_eq!(b.state(), States::Valid)
    }
}
