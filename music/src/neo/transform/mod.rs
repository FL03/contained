/*
    Appellation: transform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

pub use self::{lpr::LPR, pathfinder::*, transformer::*};

mod lpr;
mod pathfinder;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neo::triads::{Triad, Triads};

    #[test]
    fn test_pathfinder() {
        let triad = Triad::new(0.into(), Triads::Major);
        for i in [1, 3, 11] {
            let mut pathfinder = PathFinder::new(i.into()).origin(triad.clone());
            assert!(pathfinder.find().is_some());
        }
    }
}
