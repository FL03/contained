/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

pub use self::state::*;

pub(crate) mod state;

pub trait Stateful<S: StateSpec>:
    Clone + Eq + Ord + PartialEq + PartialOrd + ToString + std::ops::Add
{
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
    + std::ops::Add<Output = Self>
    + std::convert::From<i64>
    + std::convert::Into<i64>
{
}
