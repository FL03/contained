/*
    Appellation: transform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        The neo-Riemannian theory introduces three primary means of transforming triad's, namely:
            (L) Leading
            (P) Parallel
            (R) Relative
        These transformations can be chained and each preserve two of the original notes, only shifting one
        More so, if the same transformation is applied back-to-back than the resulting triad is identical to the original.
        The property of enharmonics allows us to apply the transformations according to a notes assigned position, which is a modulus of 12.

        Shift by a semitone : +/- 1
        Shift by a tone: +/- 2

        number of elements + freq
*/
use super::Dirac;
use crate::neo::triads::{Triad, Triadic};
use crate::{
    intervals::{Interval, Thirds},
    Note,
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

/// [LPR::L] Preserves the minor third; shifts the remaining note by a semitone
/// [LPR::P] Preserves the perfect fifth; shifts the remaining note by a semitone
/// [LPR::R] preserves the major third in the triad and moves the remaining note by whole tone.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum LPR {
    #[default]
    L = 0,
    P = 1,
    R = 2,
}

impl LPR {
    pub fn others(&self) -> Vec<Self> {
        vec![LPR::L, LPR::P, LPR::R]
            .into_iter()
            .filter(|x| x != self)
            .collect()
    }
    pub fn transformations() -> Vec<Self> {
        vec![LPR::L, LPR::P, LPR::R]
    }
}

impl<T: Triadic> Dirac<T> for LPR {
    type Output = T;

    fn dirac(&self, arg: &mut T) -> Self::Output {
        let mut notes: [Note; 3] = arg.triad().clone();
        match arg.intervals().0 {
            Thirds::Major => match *self {
                LPR::L => notes[0] -= Interval::Semitone,
                LPR::P => notes[1] -= Interval::Semitone,
                LPR::R => notes[2] += Interval::Tone,
            },
            Thirds::Minor => match *self {
                LPR::L => notes[2] += Interval::Semitone,
                LPR::P => notes[1] += Interval::Semitone,
                LPR::R => notes[0] -= Interval::Tone,
            },
        };
        arg.update(&notes).expect("Invalid triad");
        arg.clone()
    }
}

impl std::ops::Mul<Triad> for LPR {
    type Output = Triad;

    fn mul(self, rhs: Triad) -> Self::Output {
        self.dirac(&mut rhs.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::neo::triads::{Triad, Triads};

    #[test]
    fn test_lpr() {
        let dirac = LPR::from_str("l");
        assert!(dirac.is_ok());
        assert_eq!(dirac.unwrap(), LPR::L);
    }

    #[test]
    fn test_leading() {
        let a = Triad::new(0.into(), Triads::Major);
        let mut b = LPR::L * a.clone();
        assert_ne!(a, b);
        assert_eq!(b, Triad::try_from((4, 7, 11)).unwrap());
        b *= LPR::L;
        assert_eq!(a, b);
    }

    #[test]
    fn test_parallel() {
        let a = Triad::new(0.into(), Triads::Major);
        let b = LPR::P * a.clone();
        assert_ne!(a, b);
        assert_eq!(b, Triad::try_from((0, 3, 7)).unwrap());
        assert_eq!(LPR::P * b, a)
    }

    #[test]
    fn test_relative() {
        let a = Triad::new(0.into(), Triads::Major);
        let b = LPR::R * a.clone();

        assert_ne!(a, b);
        assert_eq!(b, Triad::try_from((9, 0, 4)).unwrap());
        assert_eq!(LPR::R * b, a)
    }
}
