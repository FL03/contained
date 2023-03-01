/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

use decanter::prelude::{hasher, Hashable, H256};
use scsys::prelude::Timestamp;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

pub trait Stateful<S: StateSpec>: Clone + Eq + Ord + PartialEq + PartialOrd + ToString {
    fn state(&self) -> &S;
}

pub trait StateSpec:
    Clone
    + Copy
    + Eq
    + Ord
    + PartialEq
    + PartialOrd
    + ToString
    + serde::Serialize
    + std::convert::From<i64>
    + std::convert::Into<i64>
{
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct State<S: StateSpec = States> {
    state: S,
    ts: i64,
}

impl<S: StateSpec> State<S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            ts: Timestamp::default().into(),
        }
    }
    pub fn timestamp(&self) -> i64 {
        self.ts
    }
    pub fn update(&mut self, state: S) {
        self.state = state;
        self.ts = Timestamp::default().into();
    }
}

impl<S: StateSpec> Hashable for State<S> {
    fn hash(&self) -> H256 {
        hasher(&self).into()
    }
}

impl<S: StateSpec> std::fmt::Display for State<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl<S: StateSpec> Stateful<S> for State<S> {
    fn state(&self) -> &S {
        &self.state
    }
}

impl<S: StateSpec> From<S> for State<S> {
    fn from(data: S) -> Self {
        Self::new(data)
    }
}

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
pub enum States {
    #[default]
    Valid = 0,
    Invalid = 1,
}

impl States {
    pub fn invalid() -> Self {
        Self::Invalid
    }
    pub fn valid() -> Self {
        Self::Valid
    }
}

impl StateSpec for States {}

impl From<usize> for States {
    fn from(d: usize) -> Self {
        Self::from(d as i64)
    }
}

impl From<i64> for States {
    fn from(d: i64) -> Self {
        match d {
            0 => States::invalid(),
            1 => States::valid(),
            _ => States::invalid(),
        }
    }
}

impl From<States> for i64 {
    fn from(d: States) -> i64 {
        d as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state() {
        let mut a = State::<States>::default();
        let b = a.clone();
        assert_eq!(a.state().clone(), States::valid());

        a.update(States::invalid());

        assert_eq!(a.state().clone(), States::invalid());
        assert_ne!(b.timestamp(), a.timestamp())
    }
}
