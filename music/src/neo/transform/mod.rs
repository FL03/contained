/*
    Appellation: transform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

pub use self::{lpr::LPR, transformer::*};

mod lpr;
mod transformer;

/// [Dirac] is a trait used to describe a transformative function;
/// Often, this trait is used to describe a set of functions that are used to transform one object into another of the same type.
pub trait Dirac<T> {
    type Output;
    /// The function that transforms the object
    fn dirac(&self, arg: &mut T) -> Self::Output;
}

/// [Transform] is a trait used to describe a type that can be transformed by a [Dirac] function.
pub trait Transform: Sized {
    type Dirac: Dirac<Self, Output = Self>;

    fn transform(&mut self, dirac: Self::Dirac) -> Self {
        dirac.dirac(self)
    }
}

