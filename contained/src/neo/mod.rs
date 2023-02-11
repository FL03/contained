/*
    Appellation: neo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        This module is dedicated to the proposed harmonic computational fabric
*/
pub use self::{tonnetz::*, transform::*, triad::*};

pub(crate) mod tonnetz;
pub(crate) mod transform;
pub(crate) mod triad;

pub mod cmp;

use futures::Stream;

pub const SEMITONE: usize = 1;
pub const TONE: usize = 2;

/// [Dirac] describes a function, [Fn] which takes one object and produces another
pub type Dirac<S, T> = dyn Fn(S) -> T;

/// A type alias for a [Stream] of [Fn] which takes in one object and transforms it into another
/// as defined in Clifton Callender's work on continuous transformations.
pub type HarmonicInterpolation<S, T> = dyn Stream<Item = Dirac<S, T>>;

pub trait Transformation<S: Clone> {
    type Error;
    type Output;

    fn args(&self) -> &S;
    /// [Transformation::dirac] represents a single function capable of transforming one object into another while allowing for errors and null results
    fn dirac(&self) -> &Dirac<&S, Result<Option<Self::Output>, Self::Error>>;
    /// [Transformation::transform] applies the defined transition to the given objects
    fn transform(&self) -> Result<Option<Self::Output>, Self::Error> {
        self.dirac()(self.args())
    }
}
