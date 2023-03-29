/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A triad is a certain type of chord built with thirds. Traditionally, this means that the triad is composed of three notes called chord factors.
        These chord factors are considered by position and are referenced as the root, third, and fifth.

        Computationally, a triadic structure is a stateful set of three notes or symbols that are related by a specific interval.

*/
use super::{tonic::Tonic, Surface, TriadClass, Triadic};
use crate::{
    intervals::{Fifths, Interval, Thirds},
    neo::{Dirac, Transform, LPR},
    Gradient, MusicError, Note,
};
use algae::graph::{Graph, UndirectedGraph};
use contained_core::{turing::Program, Alphabet, State};
use decanter::prelude::{hasher, Hashable, H256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Triads {
    Augmented([Note; 3]),
    Diminished([Note; 3]),
    Major([Note; 3]),
    Minor([Note; 3]),
}

impl Triads {
    pub fn new(root: Note, class: TriadClass) -> Self {
        let (a, b, c): (Thirds, Thirds, Fifths) = class.into();
        let notes = [root.clone(), a + root.clone(), c + root];
        match a {
            Thirds::Major => match b {
                Thirds::Major => Self::Augmented(notes),
                Thirds::Minor => Self::Major(notes),
            },
            Thirds::Minor => match b {
                Thirds::Major => Self::Minor(notes),
                Thirds::Minor => Self::Diminished(notes),
            },
        }
    }
    pub fn build(root: Note, a: Thirds, b: Thirds) -> Self {
        let notes = [root.clone(), a + root.clone(), b + (a + root)];
        match a {
            Thirds::Major => match b {
                Thirds::Major => Self::Augmented(notes),
                Thirds::Minor => Self::Major(notes),
            },
            Thirds::Minor => match b {
                Thirds::Major => Self::Minor(notes),
                Thirds::Minor => Self::Diminished(notes),
            },
        }
    }
}

impl Transform for Triads {
    type Dirac = LPR;

    fn transform(&mut self, dirac: Self::Dirac) -> Self {
        // let mut triad = self.clone();
        dirac.dirac(&mut self.clone())
    }
}

impl AsRef<[Note; 3]> for Triads {
    fn as_ref(&self) -> &[Note; 3] {
        match self {
            Triads::Augmented(notes) => notes,
            Triads::Diminished(notes) => notes,
            Triads::Major(notes) => notes,
            Triads::Minor(notes) => notes,
        }
    }
}

impl AsMut<[Note; 3]> for Triads {
    fn as_mut(&mut self) -> &mut [Note; 3] {
        match self {
            Triads::Augmented(notes) => notes,
            Triads::Diminished(notes) => notes,
            Triads::Major(notes) => notes,
            Triads::Minor(notes) => notes,
        }
    }
}

impl Triadic for Triads {
    fn class(&self) -> TriadClass {
        match self {
            Triads::Augmented(_) => TriadClass::Augmented,
            Triads::Diminished(_) => TriadClass::Diminished,
            Triads::Major(_) => TriadClass::Major,
            Triads::Minor(_) => TriadClass::Minor,
        }
    }

    fn triad(&self) -> &[Note; 3] {
        self.as_ref()
    }

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

impl TryFrom<[Note; 3]> for Triads {
    type Error = MusicError;

    fn try_from(data: [Note; 3]) -> Result<Self, Self::Error> {
        let intervals = Interval::intervals(data.clone());
        if intervals.len() != 3 {
            return Err(MusicError::IntervalError(
                "The given notes failed to contain the necessary relationships...".into(),
            ));
        } else {
            // if intervals.contains(&Interval::MajorThird) && intervals.contains(&Interval::MinorThird) {
            //     return Ok(Triads::Major(data));
            // }
            return Ok(Triads::Major(data));
        }
    }
}

impl TryFrom<[i64; 3]> for Triads {
    type Error = MusicError;

    fn try_from(data: [i64; 3]) -> Result<Self, Self::Error> {
        Self::try_from([
            Note::from(data[0]),
            Note::from(data[1]),
            Note::from(data[2]),
        ])
    }
}

/// [Triad] is a set of three [Notable] objects, the root, third, and fifth.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
        let class = self.class();
        if let Ok(t) = Self::try_from(triad.clone()) {
            *self = t;
            return Ok(self);
        }

        Err(MusicError::IntervalError(
            "The given notes failed to contain the necessary relationships...".into(),
        ))
    }
}

impl Hashable for Triad {
    fn hash(&self) -> H256 {
        hasher(self).into()
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
        let args = data.to_vec();
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
