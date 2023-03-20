/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A triad is a certain type of chord built with thirds. Traditionally, this means that the triad is composed of three notes called chord factors.
        These chord factors are considered by position and are referenced as the root, third, and fifth.
*/
use super::{actor::Actor, Triadic, Triads};
use crate::{
    intervals::{Fifths, Interval, Thirds},
    neo::LPR,
    BoxedError, Gradient, MusicResult, Notable, Note,
};
use algae::graph::{Graph, UndirectedGraph};
use contained_core::turing::{tapes::Tape, Machine, Operator, Program};
use decanter::prelude::{hasher, Hashable, H256};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

/// [Triad] is a set of three [Notable] objects, the root, third, and fifth.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Triad<N: Notable = Note> {
    class: Triads,
    notes: (N, N, N),
}

impl<N: Notable> Triad<N> {
    pub fn new(root: N, class: Triads) -> Self {
        let intervals: (Thirds, Thirds, Fifths) = class.into();
        Self::build(root, intervals.0, intervals.1)
    }
    /// Build a new [Triad] from a given [Notable] root and two [Thirds]
    pub fn build(root: N, a: Thirds, b: Thirds) -> Self {
        let notes = (root.clone(), a + root.clone(), b + (a + root));
        Self {
            class: Triads::from((a, b)),
            notes,
        }
    }
    /// Create a new [Actor] with the [Triad] as its alphabet
    pub fn actor(&self) -> Actor<N> {
        self.clone().into()
    }
    /// Initializes a new instance of a [Machine] configured with the current alphabet
    pub fn machine(&self, program: Program<N>) -> Machine<N> {
        Machine::new(
            program,
            Operator::new(0.into(), Default::default(), Tape::new(self.clone())),
        )
    }
}

impl<N: Notable> Triadic<N> for Triad<N> {
    fn triad(self) -> (N, N, N) {
        self.notes
    }

    fn update(&mut self, triad: (N, N, N)) -> MusicResult {
        if let Ok(t) = Self::try_from(triad) {
            *self = t;
            return Ok(());
        }
        Err("The given notes failed to contain the necessary relationships...".into())
    }
}

impl<N: Notable> Hashable for Triad<N> {
    fn hash(&self) -> H256 {
        hasher(self).into()
    }
}

impl<N: Notable> IntoIterator for Triad<N> {
    type Item = N;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.root(), self.third(), self.fifth()].into_iter()
    }
}

impl<N: Notable> std::fmt::Display for Triad<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.root(), self.third(), self.fifth())
    }
}

impl<N: Notable> std::ops::Mul<LPR> for Triad<N> {
    type Output = Triad<N>;

    fn mul(self, rhs: LPR) -> Self::Output {
        rhs.transform(&mut self.clone())
    }
}

impl<N: Notable> std::ops::MulAssign<LPR> for Triad<N> {
    fn mul_assign(&mut self, rhs: LPR) {
        self.transform(rhs)
    }
}

impl<N: Notable> From<(N, Thirds, Thirds)> for Triad<N> {
    fn from(data: (N, Thirds, Thirds)) -> Self {
        Self::build(data.0, data.1, data.2)
    }
}

impl<N: Notable> TryFrom<(N, N, N)> for Triad<N> {
    type Error = BoxedError;

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
                let (ab, bc) = (
                    Thirds::try_from((a.clone(), b.clone())),
                    Thirds::try_from((b.clone(), c.clone())),
                );
                // Creates a triad if the two intervals of [root, third], [third, fifth] are both considered thirds
                if ab.is_ok() && bc.is_ok() {
                    return Ok(Triad::build(a, ab?, bc?));
                }
            }
        }
        Err("Failed to find the required relationships within the given notes...".into())
    }
}

impl<N: Notable> TryFrom<(i64, i64, i64)> for Triad<N> {
    type Error = BoxedError;

    fn try_from(data: (i64, i64, i64)) -> Result<Triad<N>, Self::Error> {
        let notes: (N, N, N) = (
            data.0.pitch().into(),
            data.1.pitch().into(),
            data.2.pitch().into(),
        );
        Triad::try_from(notes)
    }
}

impl<N: Notable> From<Triad<N>> for UndirectedGraph<N, Interval> {
    fn from(d: Triad<N>) -> UndirectedGraph<N, Interval> {
        let (rt, tf, rf): (Thirds, Thirds, Fifths) = d.intervals().expect("Invalid Triad");

        let mut cluster = UndirectedGraph::with_capacity(3);
        cluster.add_edge((d.root(), d.third(), rt.into()).into());
        cluster.add_edge((d.third(), d.fifth(), tf.into()).into());
        cluster.add_edge((d.root(), d.fifth(), rf.into()).into());
        cluster.clone()
    }
}

impl<N: Notable> From<Triad<N>> for (N, N, N) {
    fn from(d: Triad<N>) -> (N, N, N) {
        (d.root(), d.third(), d.fifth())
    }
}

impl<N: Notable> From<Triad<N>> for (i64, i64, i64) {
    fn from(d: Triad<N>) -> (i64, i64, i64) {
        (d.root().pitch(), d.third().pitch(), d.fifth().pitch())
    }
}

impl<N: Notable> TryFrom<Triad<N>> for (Thirds, Thirds, Fifths) {
    type Error = BoxedError;

    fn try_from(data: Triad<N>) -> Result<(Thirds, Thirds, Fifths), Self::Error> {
        data.intervals()
    }
}

impl<N: Notable> TryFrom<Triad<N>> for (Interval, Interval, Interval) {
    type Error = BoxedError;

    fn try_from(data: Triad<N>) -> Result<(Interval, Interval, Interval), Self::Error> {
        let (a, b, c): (Thirds, Thirds, Fifths) = data.intervals()?;
        Ok((a.into(), b.into(), c.into()))
    }
}
