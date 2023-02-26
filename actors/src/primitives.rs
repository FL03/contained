/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{constants::*, types::*};

/// Simple trait for compatible symbols
pub trait Symbolic: Clone + Default + Eq + PartialEq + ToString + serde::Serialize {}

impl Symbolic for char {}

impl Symbolic for &str {}

impl Symbolic for String {}

pub(crate) mod constants {}

pub(crate) mod types {

    /// Dirac is a generic [Fn] which transforms one object into another
    pub type Dirac<S, T> = dyn Fn(S) -> T;
    /// Type alias for a [Result]
    pub type Resultant<T = (), E = String> = Result<T, E>;
}
