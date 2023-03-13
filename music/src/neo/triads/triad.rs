/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A triad is a certain type of chord built with thirds. Traditionally, this means that the triad is composed of three notes called chord factors.
        These chord factors are considered by position and are referenced as the root, third, and fifth.
*/
use super::Triads;
use crate::{
    intervals::{Fifths, Interval, Thirds},
    neo::LPR,
    Gradient, MusicResult, Notable, Note,
};
use algae::graph::{Graph, UndirectedGraph};
use contained_core::{
    turing::{Machine, Operator, Tapes},
    Scope,
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
    pub fn fifth(&self) -> N {
        self.2.clone()
    }
    pub fn root(&self) -> N {
        self.0.clone()
    }
    pub fn third(&self) -> N {
        self.1.clone()
    }
    /// Build a new [Triad] from a given [Notable] root and two [Thirds]
    pub fn build(root: N, dt: Thirds, df: Thirds) -> Self {
        let third = dt + root.clone();
        let fifth = df + third.clone();
        Self(root, third, fifth)
    }
    /// Classifies the [Triad] by describing the intervals that connect the notes
    pub fn classify(&self) -> MusicResult<(Thirds, Thirds, Fifths)> {
        let edges: (Thirds, Thirds, Fifths) = (
            Thirds::try_from((self.root(), self.third()))?,
            Thirds::try_from((self.third(), self.fifth()))?,
            Fifths::try_from((self.root(), self.fifth()))?,
        );
        Ok(edges)
    }
    /// Create a new [Operator] with the [Triad] as its alphabet
    pub fn config(&self) -> Operator<N> {
        Operator::build(Tapes::norm(self.clone()))
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
    /// Asserts the validity of a [Triad] by trying to describe it in-terms of [Thirds]
    pub fn is_valid(&self) -> bool {
        self.classify().is_ok()
    }
    /// Apply a single [LPR] transformation onto the active machine
    /// For convenience, [std::ops::Mul] was implemented as a means of applying the transformation
    pub fn transform(&mut self, dirac: LPR) {
        let (mut r, mut t, mut f): (i64, i64, i64) = self.clone().into();
        match self.classify().expect("Invalid triad").0 {
            Thirds::Major => match dirac {
                LPR::L => r -= Interval::Semitone,
                LPR::P => t -= Interval::Semitone,
                LPR::R => f += Interval::Tone,
            },
            Thirds::Minor => match dirac {
                LPR::L => f += Interval::Semitone,
                LPR::P => t += Interval::Semitone,
                LPR::R => r -= Interval::Tone,
            },
        };
        *self = Self::try_from((r, t, f)).expect("Invalid triad");
    }
    pub fn triad(self) -> (N, N, N) {
        (self.0, self.1, self.2)
    }
    /// Applies multiple [LPR] transformations onto the scoped [Triad]
    /// The goal here is to allow the machine to work on and in the scope
    pub fn walk(&mut self, iter: impl IntoIterator<Item = LPR>) {
        for dirac in iter {
            self.transform(dirac);
        }
    }
    /// Applies a set of [LPR] transformations from left-to-right, then returns home applying the same transformations in reverse
    pub fn yoyo(&mut self, iter: impl Clone + IntoIterator<Item = LPR>) {
        self.walk(iter.clone());
        let mut args = Vec::from_iter(iter);
        args.reverse();
        self.walk(args);
    }
    pub fn update(&mut self, triad: (N, N, N)) -> MusicResult {
        match Self::try_from(triad) {
            Ok(v) => {
                *self = v;
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
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
        rhs.transform(&mut self.clone())
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
    type Error = crate::BoxedError;

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
    type Error = crate::BoxedError;

    fn try_from(data: (i64, i64, i64)) -> Result<Triad<N>, Self::Error> {
        let notes: (N, N, N) = (
            data.0.pitch().into(),
            data.1.pitch().into(),
            data.2.pitch().into(),
        );
        Triad::try_from(notes)
    }
}

impl<N: Notable> TryFrom<Triad<N>> for (Thirds, Thirds, Fifths) {
    type Error = crate::BoxedError;

    fn try_from(data: Triad<N>) -> Result<(Thirds, Thirds, Fifths), Self::Error> {
        data.classify()
    }
}

impl<N: Notable> TryFrom<Triad<N>> for (Interval, Interval, Interval) {
    type Error = crate::BoxedError;

    fn try_from(data: Triad<N>) -> Result<(Interval, Interval, Interval), Self::Error> {
        let (a, b, c): (Thirds, Thirds, Fifths) = data.classify()?;
        Ok((a.into(), b.into(), c.into()))
    }
}

impl<N: Notable> From<Triad<N>> for UndirectedGraph<N, Interval> {
    fn from(d: Triad<N>) -> UndirectedGraph<N, Interval> {
        let (r, t, f): (N, N, N) = d.clone().into();
        let (rt, tf, rf): (Interval, Interval, Interval) =
            Triads::try_from(d.clone()).expect("Invalid triad").into();

        let mut cluster = UndirectedGraph::new();
        cluster.add_edge((r.clone(), t.clone(), rt));
        cluster.add_edge((t, f.clone(), tf));
        cluster.add_edge((r, f, rf));
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
        (d.0.pitch(), d.1.pitch(), d.2.pitch())
    }
}
