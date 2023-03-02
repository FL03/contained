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

use crate::Dirac;

pub trait Transformable {}

pub trait Transformation<S: Clone> {
    type Output;

    /// [Transformation::dirac] represents a single function capable of transforming one object into another while allowing for errors and null results
    fn dirac(&self) -> &Dirac<&S, Self::Output>;
    /// [Transformation::transform] applies the defined transition to the given objects
    fn transform(&self, args: &S) -> Self::Output {
        self.dirac()(args)
    }
}
