/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A triad is a certain type of chord built with thirds. Traditionally, this means that the triad is composed of three notes called chord factors.
        These chord factors are considered by position and are referenced as the root, third, and fifth.
*/
use super::Triads;
use crate::{
    intervals::{Fifths, Thirds},
    neo::LPR,
    Gradient, Notable, Note,
};
use contained_core::{
    turing::{Machine, Operator, Tapes},
    Resultant, Scope, Symbolic,
};
use serde::{Deserialize, Serialize};
use std::ops::{Mul, MulAssign};

pub trait Triadic<N: Notable>:
    Clone
    + Into<(N, N, N)>
    + IntoIterator<Item = N, IntoIter = std::vec::IntoIter<N>>
    + Mul<LPR>
    + MulAssign<LPR>
    + ToString
{
    /// Build a new [Triad] from a given [Notable] root and two [Thirds]
    fn build(root: N, dt: Thirds, df: Thirds) -> Self;
    /// Classifies the [Triad] by describing the intervals that connect the notes
    fn classify(&self) -> Resultant<(Thirds, Thirds, Fifths)> {
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
        self.into()
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

/// [Triad] is a set of three [Notable] objects, the root, third, and fifth.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Triad<N: Notable = Note>(N, N, N);

impl<N: Notable> Triad<N> {
    pub fn new(root: N, class: Triads) -> Self {
        let intervals: (Thirds, Thirds, Fifths) = class.into();
        Self::build(root, intervals.0, intervals.1)
    }
}

impl<N: Notable> Triadic<N> for Triad<N> {
    /// Build a new [Triad] from a given [Notable] root and two [Thirds]
    fn build(root: N, dt: Thirds, df: Thirds) -> Self {
        let third = dt + root.clone();
        let fifth = df + third.clone();
        Self(root, third, fifth)
    }
    fn fifth(self) -> N {
        self.2
    }
    fn root(self) -> N {
        self.0
    }
    fn third(self) -> N {
        self.1
    }
    fn transform(&mut self, dirac: LPR) {
        *self = dirac.transform(self.clone());
    }
}

impl<N: Notable> Symbolic for Triad<N> {}

impl<N: Notable> std::fmt::Display for Triad<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

impl<N: Notable> std::ops::Mul<LPR> for Triad<N> {
    type Output = Triad<N>;

    fn mul(self, rhs: LPR) -> Self::Output {
        rhs.transform(self)
    }
}

impl<N: Notable> std::ops::MulAssign<LPR> for Triad<N> {
    fn mul_assign(&mut self, rhs: LPR) {
        self.transform(rhs)
    }
}

impl<N: Notable> IntoIterator for Triad<N> {
    type Item = N;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.0, self.1, self.2].into_iter()
    }
}

impl<N: Notable> TryFrom<(N, N, N)> for Triad<N> {
    type Error = String;

    fn try_from(data: (N, N, N)) -> Result<Triad<N>, Self::Error> {
        let args = vec![data.0, data.1, data.2];
        for i in 0..args.len() {
            let tmp = [(i + 1) % args.len(), (i + 2) % args.len()];
            for j in 0..tmp.len() {
                let (a, b, c) = (
                    args[i].clone(),
                    args[tmp[j]].clone(),
                    args[tmp[(j + 1) % tmp.len()]].clone(),
                );
                // Creates a triad if the two intervals of [root, third], [third, fifth] are both considered thirds
                if Thirds::try_from((a.clone(), b.clone())).is_ok()
                    && Thirds::try_from((b.clone(), c.clone())).is_ok()
                {
                    return Ok(Triad(a, b, c));
                }
            }
        }
        Err("Failed to find the required relationships within the given notes...".into())
    }
}

impl<N: Notable> TryFrom<(i64, i64, i64)> for Triad<N> {
    type Error = String;
    fn try_from(data: (i64, i64, i64)) -> Result<Triad<N>, Self::Error> {
        let notes: (N, N, N) = (
            data.0.pitch().into(),
            data.1.pitch().into(),
            data.2.pitch().into(),
        );
        Triad::try_from(notes)
    }
}

impl<N: Notable> From<Triad<N>> for (N, N, N) {
    fn from(d: Triad<N>) -> (N, N, N) {
        (d.clone().root(), d.clone().third(), d.fifth())
    }
}

impl<N: Notable> From<Triad<N>> for (i64, i64, i64) {
    fn from(d: Triad<N>) -> (i64, i64, i64) {
        (d.0.pitch(), d.1.pitch(), d.2.pitch())
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
