/*
    Appellation: triads <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A triad is a set of three notes, called chord factors: root, third, and fifth
        Generaically, triad's share two of its notes with three of its inversions.

        For our purposes, a triad is said to be a three-tuple (a, b, c) where the intervals [a, b] and [b, c] are both thirds.
*/
pub use self::{class::*, triad::*};

pub mod tonic;
pub(crate) mod class;
pub(crate) mod triad;

use super::LPR;
use crate::{
    intervals::{Fifths, Thirds},
    Notable, MusicResult
};
use contained_core::{
    turing::{Machine, Operator, Tapes},
    Scope, 
};
use std::ops::{Mul, MulAssign};

pub trait Triadic<N: Notable>:
    Clone
    + IntoIterator<Item = N, IntoIter = std::vec::IntoIter<N>>
    + ToString
{
    /// Build a new [Triad] from a given [Notable] root and two [Thirds]
    fn build(root: N, dt: Thirds, df: Thirds) -> Self;
    /// Classifies the [Triad] by describing the intervals that connect the notes
    fn classify(&self) -> MusicResult<(Thirds, Thirds, Fifths)> {
        let edges: (Thirds, Thirds, Fifths) = Triads::try_from(self.clone().triad())?.into();
        Ok(edges)
    }
    /// Create a new [Operator] with the [Triad] as its alphabet
    fn config(&self) -> Operator<N> {
        Operator::build(Tapes::norm(self.clone()))
    }
    /// Endlessly applies the described transformations to the [Triad]
    fn cycle(&mut self, iter: impl IntoIterator<Item = LPR>) {
        for i in Vec::from_iter(iter).iter().cycle() {
            self.transform(*i);
        }
    }
    /// Initializes a new instance of a [Machine] configured with the current alphabet
    fn machine(&self) -> Machine<N> {
        Machine::new(self.config())
    }
    /// Asserts the validity of a [Triad] by trying to describe it in-terms of [Thirds]
    fn is_valid(&self) -> bool {
        self.classify().is_ok()
    }
    ///
    fn fifth(self) -> N;
    ///
    fn root(self) -> N;
    ///
    fn third(self) -> N;
    /// Apply a single [LPR] transformation onto the active machine
    /// For convenience, [std::ops::Mul] was implemented as a means of applying the transformation
    fn transform(&mut self, dirac: LPR);
    fn triad(self) -> (N, N, N) {
        (self.clone().root(), self.clone().third(), self.fifth())
    }
    /// Applies multiple [LPR] transformations onto the scoped [Triad]
    /// The goal here is to allow the machine to work on and in the scope
    fn walk(&mut self, iter: impl IntoIterator<Item = LPR>) {
        for dirac in iter {
            self.transform(dirac);
        }
    }
    /// Applies a set of [LPR] transformations from left-to-right, then returns home applying the same transformations in reverse
    fn yoyo(&mut self, iter: impl Clone + IntoIterator<Item = LPR>) {
        self.walk(iter.clone());
        let mut args = Vec::from_iter(iter);
        args.reverse();
        self.walk(args);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Note;

    #[test]
    fn test_triad() {
        let a = Triad::<Note>::new(0.into(), Triads::Major);
        let tmp: (i64, i64, i64) = a.clone().into();
        assert_eq!(tmp, (0, 4, 7));
        let b = Triad::try_from((11, 4, 7));
        assert!(b.is_ok());
        assert_ne!(a, b.unwrap())
    }

    #[test]
    fn test_walking() {
        let triad = Triad::<Note>::new(0.into(), Triads::Major);

        let mut a = triad.clone();
        // Apply three consecutive transformations to the scope
        a.walk(vec![LPR::L, LPR::P, LPR::R]);
        assert_eq!(a.clone(), Triad::try_from((1, 4, 8)).unwrap());
        // Apply the same transformations in reverse to go back to the original
        a.walk(vec![LPR::R, LPR::P, LPR::L]);
        assert_eq!(a.clone(), triad);
    }
}