/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A triad is a certain type of chord built with thirds. Traditionally, this means that the triad is composed of three notes called chord factors.
        These chord factors are considered by position and are referenced as the root, third, and fifth.

        Computationally, a triadic structure is a stateful set of three notes or symbols that are related by a specific interval.

*/
use super::{Triadic, Triads};
use crate::{
    intervals::{Fifths, Interval, Thirds},
    neo::{Dirac, Transform, LPR},
    BoxedError, Gradient, MusicResult, Note,
};
use algae::graph::{Graph, UndirectedGraph};
use contained_core::{
    actors::Actor,
    states::{State, Stateful},
    turing::{Driver, Machine, Program, Tape},
    Alphabet,
};
use decanter::prelude::{hasher, Hashable, H256};
use serde::{Deserialize, Serialize};

/// [Triad] is a set of three [Notable] objects, the root, third, and fifth.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Triad {
    class: Triads,
    notes: [Note; 3],
    state: State,
}

impl Triad {
    pub fn new(root: Note, class: Triads) -> Self {
        let (a, _, c): (Thirds, Thirds, Fifths) = class.into();
        Self {
            class,
            notes: [root.clone(), a + root.clone(), c + root],
            state: State::Valid,
        }
    }
    /// Build a new [Triad] from a given [Notable] root and two [Thirds]
    pub fn build(root: Note, a: Thirds, b: Thirds) -> Self {
        Self::new(root, Triads::from((a, b)))
    }
    /// Create a new [Actor] with the [Triad] as its alphabet
    pub fn actor(&self, tape: Option<Tape<Note>>) -> Actor<Note> {
        Actor::new(Program::new(self.clone(), State::Invalid), tape)
    }
    /// Initializes a new instance of a [Machine] configured with the current alphabet
    pub fn machine(&self, tape: Option<Tape<Note>>) -> Machine<Note> {
        Machine::new(
            Driver::new(self.state(), tape.unwrap_or_default()),
            Program::new(self.clone(), State::Invalid),
        )
    }
}

impl Alphabet<Note> for Triad {
    fn in_alphabet(&self, symbol: &Note) -> bool {
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

impl Stateful<State> for Triad {
    fn state(&self) -> State {
        self.state
    }
    fn update_state(&mut self, state: State) {
        self.state = state;
    }
}

impl Transform for Triad {
    type Dirac = LPR;
}

impl Triadic for Triad {
    fn class(&self) -> Triads {
        self.class
    }

    fn triad(&self) -> &[Note; 3] {
        &self.notes
    }

    fn update(&mut self, triad: (Note, Note, Note)) -> MusicResult {
        if let Ok(t) = Self::try_from(triad) {
            *self = t;
            return Ok(());
        }
        Err("The given notes failed to contain the necessary relationships...".into())
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
        rhs.transform(&mut self.clone())
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
    type Error = BoxedError;

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
        Err("Failed to find the required relationships within the given notes...".into())
    }
}

impl TryFrom<(Note, Note, Note)> for Triad {
    type Error = BoxedError;

    fn try_from(data: (Note, Note, Note)) -> Result<Triad, Self::Error> {
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

impl TryFrom<(i64, i64, i64)> for Triad {
    type Error = BoxedError;

    fn try_from(data: (i64, i64, i64)) -> Result<Triad, Self::Error> {
        let notes: (Note, Note, Note) = (
            data.0.pitch().into(),
            data.1.pitch().into(),
            data.2.pitch().into(),
        );
        Triad::try_from(notes)
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
