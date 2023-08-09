/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! Triad
//!
//! A [Triad] is a set of three [Note]s called chord factors ([ChordFactor]) that are related by a specific interval; represented here with a [Triads] classification.
//! [Triad]s are also considered to be stateful and can be transformed into other [Triad]s with the use of [LPR] transformations.
//! In music theory, the [Triad] is a fundamental building block used to construct more complex chords.
//! Similarly, the [Triad] is used to describe an abstract topological unit-computing environment that is often used in conjuction with other persistent instances to aid in the completion of a given task.
//! The [Wolfram (2, 3) UTM](https://www.wolframscience.com/prizes/tm23) is used as justification for describing the [Triad] as a topological unit-computing environment.
use super::{ChordFactor, Triads};
use crate::neo::{Dirac, PathFinder, Transform, LPR};
use crate::prelude::{Fifths, Gradient, Interval, MusicError, Note, Thirds};
use contained_core::states::State;
use decanter::prelude::Hashable;
use futures::Future;
use itertools::Itertools;
use petgraph::{Graph, Undirected};
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut, Range};
use std::task::{self, Poll};
use strum::IntoEnumIterator;

fn constructor(data: &[Note; 3]) -> Result<Triad, MusicError> {
    for (a, b, c) in data.iter().circular_tuple_windows() {
        if let Ok(class) = Triads::try_from((*a, *b, *c)) {
            return Ok(Triad::new(*a, class));
        }
    }
    Err(MusicError::IntervalError(
        "Failed to find the required relationships within the given notes...".into(),
    ))
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Hashable, Ord, PartialEq, PartialOrd, Serialize)]
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
            notes: [root, a + root, c + root],
            state: State::default(),
        }
    }
    pub fn from_notes(notes: [Note; 3]) -> Self {
        if let Ok(triad) = constructor(&notes) {
            return triad;
        }
        Self {
            class: Default::default(),
            notes,
            state: State::Invalid,
        }
    }
    /// Build a new [Triad] from a given [Notable] root and two [Thirds]
    pub fn build(root: Note, a: Thirds, b: Thirds) -> Self {
        Self::new(root, Triads::from((a, b)))
    }
    /// Returns the [Triads] classification of the [Triad]
    pub fn class(&self) -> Triads {
        self.class
    }
    /// Returns true if the [Triad] contains the [Note]
    pub fn contains(&self, note: &Note) -> bool {
        self.clone().into_iter().contains(note)
    }
    /// Endlessly applies the described transformations to the [Triad]
    pub fn cycle(&mut self, iter: impl IntoIterator<Item = LPR>) {
        for i in Vec::from_iter(iter).iter().cycle() {
            self.transform(*i);
        }
    }
    /// Set the initial state of the [Triad]
    pub fn default_state(mut self, state: State) -> Self {
        self.state = state;
        self
    }
    /// Returns a reference to the current composition of the [Triad]
    pub fn factors(&self) -> &[Note; 3] {
        &self.notes
    }
    /// Returns an cloned instance of the note occupying the fifth
    pub fn fifth(&self) -> Note {
        self[ChordFactor::Fifth]
    }
    /// Classifies the [Triad] by describing the intervals that connect the notes
    pub fn intervals(&self) -> (Interval, Interval, Interval) {
        let (rt, tf, rf) = self.class.intervals();
        (rt.into(), tf.into(), rf.into())
    }
    /// Returns a [Vec] of all neighboring [Triad]s; the [Triad]s that are one [LPR] away from the current [Triad]
    pub fn neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::with_capacity(3);
        for i in LPR::iter() {
            let mut triad = self.clone();
            triad.transform(i);
            neighbors.push(triad);
        }
        neighbors
    }
    /// Returns a [PathFinder] that can be used to find the path between the [Triad] and the [Note]
    pub fn pathfinder(&self, note: Note) -> PathFinder {
        PathFinder::new(note).set_origin(self.clone())
    }
    /// Returns an cloned instance of the root of the triad
    pub fn root(&self) -> Note {
        self[ChordFactor::Root]
    }
    /// Returns the [State] of the [Triad]
    pub fn state(&self) -> State {
        self.state
    }
    /// Sets the current [State] of the [Triad]
    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }
    /// Returns an cloned instance of the note occupying the third
    pub fn third(&self) -> Note {
        self[ChordFactor::Third]
    }
    /// After applying the transformation, the [Triad] is updated
    pub fn update(&mut self) -> Result<Self, MusicError> {
        if let Ok(t) = constructor(self.as_ref()) {
            *self = t;
            return Ok(self.clone());
        }
        self.state.invalidate();
        Err(MusicError::IntervalError(
            "The given notes failed to contain the necessary relationships...".into(),
        ))
    }
    /// Applies multiple [LPR] transformations onto the scoped [Triad]
    /// The goal here is to allow the machine to work on and in the scope
    pub fn walk(&mut self, iter: impl IntoIterator<Item = LPR>) {
        for dirac in iter {
            self.transform(dirac);
        }
    }
    /// Applies multiple [LPR] transformations onto the scoped [Triad] and returns a vector all the previous [Triad]
    pub fn walk_across(&mut self, iter: impl IntoIterator<Item = LPR>) -> Vec<Self> {
        let mut triads = Vec::new();
        for i in iter {
            triads.push(self.clone());
            self.transform(i);
        }
        triads
    }
    /// Applies a set of [LPR] transformations from left-to-right, then returns home applying the same transformations in reverse
    pub fn yoyo(&mut self, iter: impl Clone + IntoIterator<Item = LPR>) {
        self.walk(iter.clone());
        let mut args = Vec::from_iter(iter);
        args.reverse();
        self.walk(args);
    }
}

impl AsMut<[Note; 3]> for Triad {
    fn as_mut(&mut self) -> &mut [Note; 3] {
        &mut self.notes
    }
}

impl AsRef<[Note; 3]> for Triad {
    fn as_ref(&self) -> &[Note; 3] {
        &self.notes
    }
}

impl Default for Triad {
    fn default() -> Self {
        Self::new(0.into(), Triads::Major)
    }
}

impl std::fmt::Display for Triad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.root(), self.third(), self.fifth())
    }
}

impl Future for Triad {
    type Output = Self;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        cx.waker().wake_by_ref();

        if self.state.is_valid() {
            if let Ok(t) = self.update() {
                return Poll::Ready(t);
            }
        }
        Poll::Pending
    }
}

impl Index<ChordFactor> for Triad {
    type Output = Note;

    fn index(&self, index: ChordFactor) -> &Self::Output {
        &self.notes[index as usize]
    }
}

impl IndexMut<ChordFactor> for Triad {
    fn index_mut(&mut self, index: ChordFactor) -> &mut Self::Output {
        &mut self.notes[index as usize]
    }
}

impl Index<Range<ChordFactor>> for Triad {
    type Output = [Note];

    fn index(&self, index: Range<ChordFactor>) -> &Self::Output {
        &self.notes[index.start as usize..index.end as usize]
    }
}

impl IndexMut<Range<ChordFactor>> for Triad {
    fn index_mut(&mut self, index: Range<ChordFactor>) -> &mut Self::Output {
        &mut self.notes[index.start as usize..index.end as usize]
    }
}

impl IntoIterator for Triad {
    type Item = Note;

    type IntoIter = std::array::IntoIter<Self::Item, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.notes.into_iter()
    }
}

impl Transform for Triad {
    type Dirac = LPR;
}

impl Unpin for Triad {}


impl std::ops::Mul<LPR> for Triad {
    type Output = Triad;

    fn mul(self, rhs: LPR) -> Self::Output {
        rhs.apply(&mut self.clone())
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
        constructor(&data)
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

impl From<Triad> for Graph<Note, Interval, Undirected, ChordFactor> {
    fn from(d: Triad) -> Graph<Note, Interval, Undirected, ChordFactor> {
        let (rt, tf, rf): (Interval, Interval, Interval) = d.intervals();

        let mut graph = Graph::with_capacity(3, 3);
        let r = graph.add_node(d.root());
        let t = graph.add_node(d.third());
        let f = graph.add_node(d.fifth());
        graph.add_edge(r, t, rt);
        graph.add_edge(t, f, tf);
        graph.add_edge(r, f, rf);
        graph.clone()
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
        data.class().intervals()
    }
}

impl From<Triad> for (Interval, Interval, Interval) {
    fn from(data: Triad) -> (Interval, Interval, Interval) {
        data.intervals()
    }
}
