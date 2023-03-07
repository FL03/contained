/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{absmod, Dirac, NaturalNote, PitchClass};
use contained_core::Symbolic;

pub trait Gradient: Clone + std::convert::Into<i64> {
    fn class(&self) -> PitchClass {
        self.pitch().into()
    }
    /// [Gradient::pitch] is a method for numerically representing the structure
    fn pitch(&self) -> i64 {
        absmod(self.clone().into(), 12)
    }
}

impl Gradient for i64 {}

/// [Notable] is used to designate a structure used to represent a note
pub trait Notable: Gradient + Send + Symbolic + Sync + std::convert::From<i64> {
    /// [Notable::is_natural] Simple way to detect if the pitch is natural or not
    fn is_natural(&self) -> bool {
        NaturalNote::try_from(self.pitch()).is_ok()
    }
}

pub trait Classifiable<Cls: std::convert::From<i64>> {
    fn class(&self) -> Cls;
}

pub trait Arrays<T>: Clone + ExactSizeIterator + IntoIterator<Item = T> {
    fn as_vec(self) -> Vec<T> {
        Vec::from_iter(self)
    }
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
