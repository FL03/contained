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

use super::LPR;
use crate::intervals::{Fifths, Interval, Thirds};
use crate::{MusicResult, Note};

pub trait Triadic: Clone {
    fn class(&self) -> Triads;

    /// Endlessly applies the described transformations to the [Triad]
    fn cycle(&mut self, iter: impl IntoIterator<Item = LPR>) {
        for i in Vec::from_iter(iter).iter().cycle() {
            self.transform(*i);
        }
    }
    /// Returns an cloned instance of the note occupying the fifth
    fn fifth(&self) -> Note {
        self.clone().triad().2
    }
    /// Classifies the [Triad] by describing the intervals that connect the notes
    fn intervals(&self) -> (Thirds, Thirds, Fifths) {
        self.class().intervals()
    }
    /// Returns an cloned instance of the root of the triad
    fn root(&self) -> Note {
        self.clone().triad().0
    }
    /// Returns an cloned instance of the note occupying the third
    fn third(&self) -> Note {
        self.clone().triad().1
    }
    /// Apply a single [LPR] transformation onto the active machine
    /// For convenience, [std::ops::Mul] was implemented as a means of applying the transformation
    fn transform(&mut self, dirac: LPR) {
        let (mut r, mut t, mut f): (Note, Note, Note) = self.clone().triad();
        match self.intervals().0 {
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
    ///
    fn triad(self) -> (Note, Note, Note);
    /// Applies multiple [LPR] transformations onto the scoped [Triad]
    /// The goal here is to allow the machine to work on and in the scope
    fn walk(&mut self, iter: impl IntoIterator<Item = LPR>) {
        for dirac in iter {
            self.transform(dirac);
        }
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
