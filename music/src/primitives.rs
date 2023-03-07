/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{constants::*, types::*};

pub(crate) mod constants {
    pub const SEMITONE: usize = 1;
    pub const TONE: usize = 2;
}

pub(crate) mod types {
    use futures::Stream;

    /// Dirac is a generic [Fn] which transforms one object into another
    pub type Dirac<S, T> = dyn Fn(S) -> T;
    /// A type alias for a [Stream] of [Fn] which takes in one object and transforms it into another
    /// as defined in Clifton Callender's work on continuous transformations.
    pub type HarmonicInterpolation<S, T> = dyn Stream<Item = Dirac<S, T>>;
}
