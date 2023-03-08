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
    Gradient, Notable,
};
use contained_core::{
    turing::{Machine, Operator, Tapes},
    Resultant, Scope, Symbolic,
};
use serde::{Deserialize, Serialize};

/// [Triad] is a set of three [Notable] objects, the root, third, and fifth.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Triad<N: Notable>(N, N, N);

impl<N: Notable> Triad<N> {
    pub fn new(root: N, class: Triads) -> Self {
        let (a, b) = Thirds::compute(root.clone());

        let triad = match class {
            Triads::Augmented => (root, a.clone(), Thirds::Major * a),
            Triads::Diminshed => (root, b.clone(), Thirds::Minor * b),
            Triads::Major => (root, a.clone(), Thirds::Minor * a),
            Triads::Minor => (root, b.clone(), Thirds::Major * b),
        };
        Self(triad.0, triad.1, triad.2)
    }
    ///
    pub fn classify(&self) -> Resultant<Triads> {
        Triads::try_from(self.clone())
    }
    /// Create a new [Operator] with the [Triad] as its alphabet
    pub fn config(&self) -> Operator<N> {
        Operator::build(Tapes::normal(
            Vec::from_iter(self.clone().into_iter()).into(),
        ))
    }
    /// Endlessly applies the described transformations to the [Triad]
    pub fn cycle(&mut self, iter: impl IntoIterator<Item = LPR>) {
        for i in Vec::from_iter(iter).iter().cycle() {
            self.transform(*i);
        }
    }
    /// Initializes a new instance of a [Machine] configured with the current alphabet
    pub fn machine(&self) -> Machine<N> {
        Machine::new(self.config())
    }
    /// Checks to see if the first interval is a third and the second interval is a fifth
    pub fn is_valid(&self) -> bool {
        let triad: (N, N, N) = self.clone().into();
        Thirds::try_from((triad.0, triad.1.clone())).is_ok()
            && Fifths::try_from((triad.1, triad.2)).is_ok()
    }
    ///
    pub fn fifth(self) -> N {
        self.2
    }
    ///
    pub fn root(self) -> N {
        self.0
    }
    ///
    pub fn third(self) -> N {
        self.1
    }
    /// Apply a single [LPR] transformation onto the active machine
    /// For convenience, [std::ops::Mul] was implemented as a means of applying the transformation
    pub fn transform(&mut self, dirac: LPR) {
        *self = dirac.transform(self.clone());
    }
    /// Applies multiple [LPR] transformations onto the scoped [Triad]
    /// The goal here is to allow the machine to work on and in the scope
    pub fn walk(&mut self, chain: impl IntoIterator<Item = LPR>) {
        for dirac in chain {
            self.transform(dirac);
        }
    }
}

impl<N: Eq + Notable + Ord + Serialize + std::fmt::Debug> Symbolic for Triad<N> {}

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

impl<N: Notable> From<Triad<N>> for (N, N, N) {
    fn from(d: Triad<N>) -> (N, N, N) {
        (d.0.clone(), d.1.clone(), d.2)
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
