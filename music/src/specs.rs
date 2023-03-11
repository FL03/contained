/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{absmod, Dirac, NaturalNote, PitchClass};
use contained_core::{graphs, Symbolic};

/// [Gradient] provides a numerical interpretation of a given object
pub trait Gradient: Clone + Eq + Ord + std::convert::Into<i64> {
    const MODULUS: i64;

    fn class(&self) -> PitchClass {
        self.pitch().into()
    }
    /// [Gradient::pitch] is a method for numerically representing the structure
    fn pitch(&self) -> i64 {
        absmod(self.clone().into(), Self::MODULUS)
    }
}

impl Gradient for i64 {
    const MODULUS: i64 = 12;
}

/// [Notable] is used to designate a structure used to represent a note
pub trait Notable: From<i64> + Gradient + Send + Symbolic + Sync + graphs::cmp::Node {
    /// [Notable::is_natural] Simple way to detect if the pitch is natural or not
    fn is_natural(&self) -> bool {
        NaturalNote::try_from(self.pitch()).is_ok()
    }
}

pub trait Classifiable<Cls: From<i64>> {
    fn class(&self) -> Cls;
}

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

pub trait Validity {
    fn is_valid(&self) -> bool;
}
