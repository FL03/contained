/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::state::*;

mod state;

use crate::Shared;

pub trait AsyncStateful<S: StateSpec>: Clone {
    fn state(&self) -> Shared<S>;
    fn update_state(&mut self, state: Shared<S>);
}

/// [Stateful] describes a stateful object
pub trait Stateful<S: StateSpec>: Clone {
    /// [Stateful::state] is used to get the state of the object
    fn state(&self) -> S;
    /// [Stateful::update_state] is used to update the state of the object
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
pub trait StateSpec {}

impl StateSpec for i64 {}