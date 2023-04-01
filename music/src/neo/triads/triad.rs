/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A triad is a certain type of chord built with thirds. Traditionally, this means that the triad is composed of three notes called chord factors.
        These chord factors are considered by position and are referenced as the root, third, and fifth.

        Computationally, a triadic structure is a stateful set of three notes or symbols that are related by a specific interval.

*/
use super::{Surface, Tonic, TriadClass, Triadic};
use crate::{
    intervals::{Fifths, Interval, Thirds},
    neo::{Dirac, Transform, LPR},
    Gradient, MusicError, Note,
};
use algae::graph::{Graph, UndirectedGraph};
use contained_core::{
    turing::{Alphabet, Program},
    State,
};
use decanter::prelude::Hashable;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

/// [Triad] is a set of three [Notable] objects, the root, third, and fifth.
#[derive(
    Clone, Debug, Default, Deserialize, Eq, Hash, Hashable, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Triad {
    class: TriadClass,
    notes: [Note; 3],
}

impl Triad {
    pub fn new(root: Note, class: TriadClass) -> Self {
        let (a, _, c): (Thirds, Thirds, Fifths) = class.into();
        Self {
            class,
            notes: [root.clone(), a + root.clone(), c + root],
        }
    }
    /// Build a new [Triad] from a given [Notable] root and two [Thirds]
    pub fn build(root: Note, a: Thirds, b: Thirds) -> Self {
        Self::new(root, TriadClass::from((a, b)))
    }

    pub fn instance(&self) -> Surface {
        Surface::new(self.clone())
    }

    pub fn tonic(&self, module: wasmer::Module) -> Tonic {
        Tonic::new(module, self.instance())
    }

    pub fn program(&self) -> Program<Note> {
        Program::new(self.clone(), State::Invalid)
    }
}

impl Alphabet<Note> for Triad {
    fn is_viable(&self, symbol: &Note) -> bool {
        self.notes.contains(symbol)
    }
    fn default_symbol(&self) -> Note {
        self.root()
    }
}

impl AsMut<[Note; 3]> for Triad {
    fn as_mut(&mut self) -> &mut [Note; 3] {
        &mut self.notes
    }
}

impl AsRef<[Note; 3]> for Triad {
    fn as_ref(&self) -> &[Note; 3] {
        self.triad()
    }
}

impl Transform for Triad {
    type Dirac = LPR;
}

impl Triadic for Triad {
    fn class(&self) -> TriadClass {
        self.class
    }

    fn triad(&self) -> &[Note; 3] {
        &self.notes
    }
    // TODO: "Fix the transformations; they fail to preserve the triad class during the transformation"
    fn update(&mut self, triad: &[Note; 3]) -> Result<&mut Self, MusicError> {
        if let Ok(t) = Self::try_from(triad.clone()) {
            *self = t;
            return Ok(self);
        }

        Err(MusicError::IntervalError(
            "The given notes failed to contain the necessary relationships...".into(),
        ))
    }
}

impl IntoIterator for Triad {
    type Item = Note;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.triad().to_vec().into_iter()
    }
}

impl std::fmt::Display for Triad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.root(), self.third(), self.fifth())
    }
}

impl std::ops::Mul<LPR> for Triad {
    type Output = Triad;

    fn mul(self, rhs: LPR) -> Self::Output {
        rhs.dirac(&mut self.clone())
    }
}

impl std::ops::MulAssign<LPR> for Triad {
    fn mul_assign(&mut self, rhs: LPR) {
        *self = self.transform(rhs);
    }
}

impl From<(Note, Thirds, Thirds)> for Triad {
    fn from(data: (Note, Thirds, Thirds)) -> Self {
        Self::build(data.0, data.1, data.2)
    }
}

impl TryFrom<[Note; 3]> for Triad {
    type Error = MusicError;

    fn try_from(data: [Note; 3]) -> Result<Triad, Self::Error> {
        for (a, b, c) in data.into_iter().circular_tuple_windows() {
            if let Ok(class) = TriadClass::try_from((a.clone(), b.clone(), c.clone())) {
                return Ok(Triad::new(a.clone(), class));
            }
        }
        Err(MusicError::IntervalError(
            "Failed to find the required relationships within the given notes...".into(),
        ))
    }
}

impl TryFrom<(Note, Note, Note)> for Triad {
    type Error = MusicError;

    fn try_from(data: (Note, Note, Note)) -> Result<Triad, Self::Error> {
        Triad::try_from([data.0, data.1, data.2])
    }
}

impl TryFrom<[i64; 3]> for Triad {
    type Error = MusicError;

    fn try_from(notes: [i64; 3]) -> Result<Triad, Self::Error> {
        let notes: [Note; 3] = [notes[0].into(), notes[1].into(), notes[2].into()];
        Triad::try_from(notes)
    }
}

impl TryFrom<(i64, i64, i64)> for Triad {
    type Error = MusicError;

    fn try_from(data: (i64, i64, i64)) -> Result<Triad, Self::Error> {
        Triad::try_from([data.0, data.1, data.2])
    }
}

impl From<Triad> for UndirectedGraph<Note, Interval> {
    fn from(d: Triad) -> UndirectedGraph<Note, Interval> {
        let (rt, tf, rf): (Thirds, Thirds, Fifths) = d.intervals();

        let mut cluster = UndirectedGraph::with_capacity(3);
        cluster.add_edge((d.root(), d.third(), rt.into()).into());
        cluster.add_edge((d.third(), d.fifth(), tf.into()).into());
        cluster.add_edge((d.root(), d.fifth(), rf.into()).into());
        cluster.clone()
    }
}

impl From<Triad> for Vec<Note> {
    fn from(d: Triad) -> Vec<Note> {
        vec![d.root(), d.third(), d.fifth()]
    }
}

impl From<Triad> for [Note; 3] {
    fn from(d: Triad) -> [Note; 3] {
        d.triad().clone()
    }
}

impl From<Triad> for (Note, Note, Note) {
    fn from(d: Triad) -> (Note, Note, Note) {
        (d.root(), d.third(), d.fifth())
    }
}

impl From<Triad> for (i64, i64, i64) {
    fn from(d: Triad) -> (i64, i64, i64) {
        (d.root().pitch(), d.third().pitch(), d.fifth().pitch())
    }
}

impl From<Triad> for (Thirds, Thirds, Fifths) {
    fn from(data: Triad) -> (Thirds, Thirds, Fifths) {
        data.intervals()
    }
}

impl From<Triad> for (Interval, Interval, Interval) {
    fn from(data: Triad) -> (Interval, Interval, Interval) {
        let intervals = data.intervals();
        (intervals.0.into(), intervals.1.into(), intervals.2.into())
    }
}
