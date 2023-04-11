/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A triad is a certain type of chord built with thirds. Traditionally, this means that the triad is composed of three notes called chord factors.
        These chord factors are considered by position and are referenced as the root, third, and fifth.

        Computationally, a triadic structure is a stateful set of three notes or symbols that are related by a specific interval.

*/
use super::{ChordFactor, Triads};
use crate::intervals::{Fifths, Interval, Thirds};
use crate::neo::{Dirac, PathFinder, Transform, LPR};
use crate::{Gradient, MusicError, Note};
use algae::graph::{Graph, GraphExt, UndirectedGraph};
use contained_core::states::State;
use decanter::prelude::Hashable;
use futures::Future;
use itertools::Itertools;
use petgraph::graph::UnGraph;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut, Range};
use std::task::{self, Poll};

fn constructor(data: &[Note; 3]) -> Result<Triad, MusicError> {
    for (a, b, c) in data.into_iter().circular_tuple_windows() {
        if let Ok(class) = Triads::try_from((a.clone(), b.clone(), c.clone())) {
            return Ok(Triad::new(a.clone(), class));
        }
    }
    Err(MusicError::IntervalError(
        "Failed to find the required relationships within the given notes...".into(),
    ))
}

/// [Triad] is a set of three [Notable] objects, the root, third, and fifth.
#[derive(
    Clone, Debug, Default, Deserialize, Eq, Hash, Hashable, Ord, PartialEq, PartialOrd, Serialize,
)]
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
            state: State::default(),
        }
    }
    /// Build a new [Triad] from a given [Notable] root and two [Thirds]
    pub fn build(root: Note, a: Thirds, b: Thirds) -> Self {
        Self::new(root, Triads::from((a, b)))
    }
    pub fn class(&self) -> Triads {
        self.class
    }
    /// Returns true if the [Triad] contains the [Note]
    pub fn contains(&self, note: &Note) -> bool {
        &self.root() == note || &self.third() == note || &self.fifth() == note
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
    /// Returns an cloned instance of the note occupying the fifth
    pub fn fifth(&self) -> Note {
        self[ChordFactor::Fifth].clone()
    }
    /// Classifies the [Triad] by describing the intervals that connect the notes
    pub fn intervals(&self) -> (Thirds, Thirds, Fifths) {
        self.class().intervals()
    }
    /// Returns a vector of all the possible [Triad]s that exist at
    pub fn neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::with_capacity(3);
        for i in LPR::transformations() {
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
        self[ChordFactor::Root].clone()
    }
    /// Returns the [State] of the [Triad]
    pub fn state(&self) -> State {
        self.state
    }
    /// Returns an cloned instance of the note occupying the third
    pub fn third(&self) -> Note {
        self[ChordFactor::Third].clone()
    }
    /// After applying the transformation, the [Triad] is updated
    pub fn update(&mut self) -> Result<Self, MusicError> {
        if let Ok(t) = constructor(self.as_ref()) {
            *self = t;
            return Ok(self.clone());
        } else {
            self.state.invalidate();
            Err(MusicError::IntervalError(
                "The given notes failed to contain the necessary relationships...".into(),
            ))
        }
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
    pub fn triad(&self) -> &[Note; 3] {
        &self.notes
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

impl Future for Triad {
    type Output = Self;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        if self.state.is_valid() {
            Poll::Ready(self.clone())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

impl Transform for Triad {
    type Dirac = LPR;
}

impl IntoIterator for Triad {
    type Item = Note;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.triad().to_vec().into_iter()
    }
}

impl Unpin for Triad {}

impl std::fmt::Display for Triad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.root(), self.third(), self.fifth())
    }
}

impl Index<ChordFactor> for Triad {
    type Output = Note;

    fn index(&self, index: ChordFactor) -> &Self::Output {
        use ChordFactor::*;
        match index {
            Root => &self.notes[Root as usize],
            Third => &self.notes[Third as usize],
            Fifth => &self.notes[Fifth as usize],
        }
    }
}

impl IndexMut<ChordFactor> for Triad {
    fn index_mut(&mut self, index: ChordFactor) -> &mut Self::Output {
        use ChordFactor::*;
        match index {
            Root => &mut self.notes[Root as usize],
            Third => &mut self.notes[Third as usize],
            Fifth => &mut self.notes[Fifth as usize],
        }
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
            if let Ok(class) = Triads::try_from((a.clone(), b.clone(), c.clone())) {
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
    fn from(triad: Triad) -> UndirectedGraph<Note, Interval> {
        let (rt, tf, rf): (Thirds, Thirds, Fifths) = triad.intervals();
        let mut cluster = UndirectedGraph::with_capacity(3);
        let edges = vec![
            (triad.root(), triad.third(), rt.into()).into(),
            (triad.third(), triad.fifth(), tf.into()).into(),
            (triad.root(), triad.fifth(), rf.into()).into(),
        ];
        cluster.add_edges(edges);
        cluster.clone()
    }
}

impl From<Triad> for UnGraph<Note, Interval> {
    fn from(d: Triad) -> UnGraph<Note, Interval> {
        let (rt, tf, rf): (Thirds, Thirds, Fifths) = d.intervals();

        let mut cluster = UnGraph::with_capacity(3, 3);
        let root = cluster.add_node(d.root());
        let third = cluster.add_node(d.third());
        let fifth = cluster.add_node(d.fifth());
        cluster.add_edge(root, third, rt.into());
        cluster.add_edge(third, fifth, tf.into());
        cluster.add_edge(root, fifth, rf.into());
        cluster.clone()
    }
}

impl From<Triad> for Vec<Note> {
    fn from(d: Triad) -> Vec<Note> {
        vec![d.root(), d.third(), d.fifth()]
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
