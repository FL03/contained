/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

pub use self::state::*;

pub(crate) mod state;

/// [Stateful] describes a stateful object
pub trait Stateful<S: StateSpec>:
    Clone + Eq + Ord + PartialEq + PartialOrd + ToString + std::ops::Add
{
    fn state(&self) -> &S;
}

impl Stateful<i64> for i64 {
    fn state(&self) -> &i64 {
        &self
    }
}

/// [StateSpec] is used by [Stateful] to indicate an actual state
pub trait StateSpec:
    Clone
    + Copy
    + Eq
    + Ord
    + PartialEq
    + PartialOrd
    + ToString
    + serde::Serialize
    + std::ops::Add<Output = Self>
    + std::convert::From<i64>
    + std::convert::Into<i64>
{
}

impl StateSpec for i64 {}
