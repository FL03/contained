/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

pub use self::state::*;

pub(crate) mod state;

/// [Stateful] describes a stateful object
pub trait Stateful<S: StateSpec>: Clone + Default + PartialEq + PartialOrd + ToString {
    fn state(&self) -> S;
    fn update_state(&mut self, state: S);
}

impl Stateful<i64> for i64 {
    fn state(&self) -> i64 {
        *self
    }
    fn update_state(&mut self, state: i64) {
        *self = state;
    }
}

/// [StateSpec] is used by [Stateful] to describe a specific state
pub trait StateSpec:
    Clone + Copy + Default + Eq + Ord + std::fmt::Display + std::ops::Add<Output = Self>
{
}

impl StateSpec for i64 {}
