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
