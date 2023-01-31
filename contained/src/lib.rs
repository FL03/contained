/*
    Appellation: Contained <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::states::*;

pub(crate) mod states;

pub mod turing;

pub type Dirac<S, T> = dyn Fn(S) -> T;

pub type Resultant<T = (), E = String> = Result<T, E>;

pub trait Stateful {
    type State;

    fn state(&self) -> &Self::State
    where
        Self: Sized;
}
