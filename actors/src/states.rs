/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

use decanter::prelude::{Hash, Hashable};
use scsys::prelude::{Message, SerdeDisplay, Timestamp};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

pub trait Stateful: Clone + Eq + Ord + PartialEq + PartialOrd + ToString {
    type State: StateSpec;

    fn state(&self) -> &Self::State;
}

pub trait StateSpec:
    Clone
    + Copy
    + Eq
    + Ord
    + PartialEq
    + PartialOrd
    + ToString
    + std::convert::From<i64>
    + std::convert::Into<i64>
{
    fn state(&self) -> &Self {
        &self
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct State<S: StateSpec = States> {
    state: S,
}

impl<S: StateSpec> State<S> {
    pub fn new(state: S) -> Self {
        Self { state }
    }
    pub fn update(&mut self, state: S) {
        self.state = state;
    }
}

impl<S: StateSpec> std::fmt::Display for State<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.state().to_string())
    }
}

impl<S: StateSpec> Stateful for State<S> {
    type State = S;

    fn state(&self) -> &Self::State {
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
        // assert_ne!(b.timestamp(), a.timestamp())
    }
}
