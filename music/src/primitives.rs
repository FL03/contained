/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{constants::*, types::*};

mod constants {
    /// Used to describe the total number of notes considered
    pub const MODULUS: i64 = 12;
    /// A semitone is half of a tone
    pub const SEMITONE: i64 = 1;
    /// A tone is a difference of two
    pub const TONE: i64 = 2;
}

mod types {
    use futures::Stream;

    pub type BoxedError = Box<dyn std::error::Error>;

    pub type MusicResult<T = ()> = Result<T, BoxedError>;
    /// Dirac is a generic [Fn] which transforms one object into another
    pub type Dirac<S, T> = dyn Fn(S) -> T;
    /// A type alias for a [Stream] of [Fn] which takes in one object and transforms it into another
    /// as defined in Clifton Callender's work on continuous transformations.
    pub type HarmonicInterpolation<S, T> = dyn Stream<Item = Dirac<S, T>>;
}
