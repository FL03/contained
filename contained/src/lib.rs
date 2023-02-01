/*
    Appellation: Contained <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::states::*;

pub(crate) mod states;

pub mod neo;
pub mod turing;

/// Dirac is a generic [Fn] which transforms one object into another
pub type Dirac<S, T> = dyn Fn(S) -> T;
/// Type alias for a [Result]
pub type Resultant<T = (), E = String> = Result<T, E>;

/// Simple trait for signaling a [Stateful]
pub trait Stateful {
    type State;

    fn state(&self) -> &Self::State
    where
        Self: Sized;
}

/// A trait for implementing state transitions
pub trait Transition<S: Clone> {
    type Output;

    fn data(&self) -> &S;
    fn dirac(&self) -> &Dirac<S, Self::Output>;
    fn resultant(&self) -> Self::Output {
        self.dirac()(self.data().clone())
    }
}
