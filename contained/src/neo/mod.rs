/*
    Appellation: neo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        This module is dedicated to the proposed harmonic computational fabric
*/
pub use self::{tonnetz::*, triad::*};

pub(crate) mod tonnetz;
pub(crate) mod triad;

pub mod cmp;

use futures::Stream;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

/// A type alias for a [Stream] of [Fn] which takes in one object and transforms it into another
/// as defined in Clifton Callender's work on continuous transformations.
pub type HarmonicInterpolation<S, T> = dyn Stream<Item = dyn Fn(S) -> T>;

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum Transform {
    #[default]
    L = 0,
    P = 1,
    R = 2,
}
