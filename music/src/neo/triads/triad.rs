/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A triad is a certain type of chord built with thirds. Traditionally, this means that the triad is composed of three notes called chord factors.
        These chord factors are considered by position and are referenced as the root, third, and fifth.
*/
use super::{Triadic, Triads};
use crate::{
    intervals::{Fifths, Thirds},
    neo::LPR,
    Gradient, Notable, Note, MusicResult
};
use decanter::prelude::{hasher, Hashable, H256};
use serde::{Deserialize, Serialize};

/// [Triad] is a set of three [Notable] objects, the root, third, and fifth.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Triad<N: Notable = Note>(N, N, N);

impl<N: Notable> Triad<N> {
    pub fn new(root: N, class: Triads) -> Self {
        let intervals: (Thirds, Thirds, Fifths) = class.into();
        Self::build(root, intervals.0, intervals.1)
    }
    pub fn update(&mut self, triad: (N, N, N)) -> MusicResult {
        match Self::try_from(triad) {
            Ok(v) => {
                *self = v;
                Ok(())
            },
            Err(e) => Err(e.into())
        }
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

impl<N: Notable> Hashable for Triad<N> {
    fn hash(&self) -> H256 {
        hasher(self).into()
    }
}

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
