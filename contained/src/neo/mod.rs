/*
    Appellation: neo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        This module is dedicated to the proposed harmonic computational fabric
*/
pub use self::{note::*, tonnetz::*, triad::*};

pub(crate) mod note;
pub(crate) mod tonnetz;
pub(crate) mod triad;

use futures::Stream;

/// A type alias for a [Stream] of [Fn] which take one object and transform it into another
/// as defined in Clifton Callender's work on continuous transformations.
pub type HarmonicInterpolation<S, T> = dyn Stream<Item = dyn Fn(S) -> T>;
