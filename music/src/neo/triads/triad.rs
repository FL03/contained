/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A triad is a certain type of chord built with thirds. Traditionally, this means that the triad is composed of three notes called chord factors.
        These chord factors are considered by position and are referenced as the root, third, and fifth.
*/
use super::{actor::Actor, Triads};
use crate::{
    intervals::{Fifths, Interval, Thirds},
    neo::LPR,
    BoxedError, Gradient, MusicResult, Notable, Note,
};
use algae::graph::{Graph, UndirectedGraph};
use contained_core::turing::{tapes::Tape, Machine, Operator, Program};
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
    /// Build a new [Triad] from a given [Notable] root and two [Thirds]
    pub fn build(root: N, a: Thirds, b: Thirds) -> Self {
        Self(root.clone(), a + root.clone(), b + (a + root))
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
    /// Create a new [Actor] with the [Triad] as its alphabet
    pub fn actor(&self) -> Actor<N> {
        self.clone().into()
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
    /// Endlessly applies the described transformations to the [Triad]
    pub fn cycle(&mut self, iter: impl IntoIterator<Item = LPR>) {
        for i in Vec::from_iter(iter).iter().cycle() {
            self.transform(*i);
        }
    }
    /// Initializes a new instance of a [Machine] configured with the current alphabet
    pub fn machine(&self, program: Program<N>) -> Machine<N> {
        Machine::new(
            Operator::new(0.into(), Default::default(), Tape::new(self.clone())),
            program,
        )
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
        self.update((r.into(), t.into(), f.into()))
            .expect("Invalid triad");
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

impl<N: Notable> IntoIterator for Triad<N> {
    type Item = N;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.0, self.1, self.2].into_iter()
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
        let (rt, tf, rf): (Thirds, Thirds, Fifths) = d.classify().expect("Invalid Triad");

        let mut cluster = UndirectedGraph::new();
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
        (d.0.pitch(), d.1.pitch(), d.2.pitch())
    }
}

impl<N: Notable> TryFrom<Triad<N>> for (Thirds, Thirds, Fifths) {
    type Error = BoxedError;

    fn try_from(data: Triad<N>) -> Result<(Thirds, Thirds, Fifths), Self::Error> {
        data.classify()
    }
}

impl<N: Notable> TryFrom<Triad<N>> for (Interval, Interval, Interval) {
    type Error = BoxedError;

    fn try_from(data: Triad<N>) -> Result<(Interval, Interval, Interval), Self::Error> {
        let (a, b, c): (Thirds, Thirds, Fifths) = data.classify()?;
        Ok((a.into(), b.into(), c.into()))
    }
}
