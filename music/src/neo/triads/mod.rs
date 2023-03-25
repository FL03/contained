/*
    Appellation: triads <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A triad is a set of three notes, called chord factors: root, third, and fifth
        Generaically, triad's share two of its notes with three of its inversions.

        For our purposes, a triad is said to be a three-tuple (a, b, c) where the intervals [a, b] and [b, c] are both thirds.
*/

pub use self::{class::*, triad::*};

pub(crate) mod class;
pub(crate) mod triad;

pub mod tonic;

use super::{Transform, LPR};
use crate::intervals::{Fifths, Interval, Thirds};
use crate::{MusicResult, Note};
use std::collections::HashMap;

pub trait Triadic: AsRef<[Note; 3]> + Clone + Transform<Dirac = LPR> {
    fn class(&self) -> Triads;
    fn contains(&self, note: &Note) -> bool {
        &self.root() == note || &self.third() == note || &self.fifth() == note
    }
    /// Endlessly applies the described transformations to the [Triad]
    fn cycle(&mut self, iter: impl IntoIterator<Item = LPR>) {
        for i in Vec::from_iter(iter).iter().cycle() {
            self.transform(*i);
        }
    }
    /// Returns an cloned instance of the note occupying the fifth
    fn fifth(&self) -> Note {
        self.triad()[2].clone()
    }
    fn get_path_to(&self, note: &Note) -> Option<Vec<LPR>> {
        let mut queue = vec![(Vec::new(), self.clone())];
        while let Some((path, triad)) = queue.pop() {
            if triad.contains(note) {
                return Some(path);
            }
            for i in LPR::transformations() {
                let mut triad = triad.clone();
                triad.transform(i);
                let mut path = path.clone();
                path.push(i);
                if triad.contains(note) {
                    return Some(path);
                }
                queue.push((path, triad));
            }
        }
        None
    }
    /// Classifies the [Triad] by describing the intervals that connect the notes
    fn intervals(&self) -> (Thirds, Thirds, Fifths) {
        self.class().intervals()
    }
    fn neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::with_capacity(3);
        for i in LPR::transformations() {
            let mut triad = self.clone();
            triad.transform(i);
            neighbors.push(triad);
        }
        neighbors
    }
    /// Returns an cloned instance of the root of the triad
    fn root(&self) -> Note {
        self.triad()[0].clone()
    }
    /// Returns an cloned instance of the note occupying the third
    fn third(&self) -> Note {
        self.triad()[1].clone()
    }
    ///
    fn triad(&self) -> &[Note; 3];
    /// Applies multiple [LPR] transformations onto the scoped [Triad]
    /// The goal here is to allow the machine to work on and in the scope
    fn walk(&mut self, iter: impl IntoIterator<Item = LPR>) {
        for dirac in iter {
            self.transform(dirac);
        }
    }
    /// Applies multiple [LPR] transformations onto the scoped [Triad] and returns a vector all the previous [Triad]
    fn walk_across(&mut self, iter: impl IntoIterator<Item = LPR>) -> Vec<Self> {
        let mut triads = Vec::new();
        for i in iter {
            triads.push(self.clone());
            self.transform(i);
        }
        triads
    }
    /// Applies a set of [LPR] transformations from left-to-right, then returns home applying the same transformations in reverse
    fn yoyo(&mut self, iter: impl Clone + IntoIterator<Item = LPR>) {
        self.walk(iter.clone());
        let mut args = Vec::from_iter(iter);
        args.reverse();
        self.walk(args);
    }
    fn update(&mut self, triad: (Note, Note, Note)) -> MusicResult;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neo::LPR;

    #[test]
    fn test_triad() {
        let a = Triad::new(0.into(), Triads::Major);
        let tmp: (i64, i64, i64) = a.clone().into();
        assert_eq!(tmp, (0, 4, 7));
        let b = Triad::try_from((11, 4, 7));
        assert!(b.is_ok());
        assert_ne!(a, b.unwrap())
    }

    #[test]
    fn test_find_path() {
        let triad = Triad::new(0.into(), Triads::Major);
        assert!(triad.get_path_to(&Note::from(3)).is_some());
        // assert!(triad.get_path_to(&Note::from(1)).is_some());
        // assert!(triad.get_path_to(&Note::from(11)).is_some());
    }

    #[test]
    fn test_walking() {
        let expected = Triad::try_from((1, 4, 8)).unwrap();
        let triad = Triad::new(0.into(), Triads::Major);

        let mut a = triad.clone();
        let mut b = triad.clone();
        // Apply three consecutive transformations to the scope
        a.walk(vec![LPR::L, LPR::P, LPR::R]);
        assert_eq!(a.clone(), expected);
        // Apply the same transformations in reverse to go back to the original
        a.walk(vec![LPR::R, LPR::P, LPR::L]);
        b.yoyo(vec![LPR::L, LPR::P, LPR::R]);
        assert_eq!(a, b);
    }
}
