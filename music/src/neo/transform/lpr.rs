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
use crate::intervals::{Interval, Thirds};
use crate::neo::triads::{ChordFactor, Triad};
use decanter::prelude::Hashable;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString, EnumVariantNames, IntoEnumIterator};

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
    EnumIter,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Hashable,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum LPR {
    #[default]
    #[strum(serialize = "l", serialize = "leading")]
    L = 0,
    #[strum(serialize = "p", serialize = "parallel")]
    P = 1,
    #[strum(serialize = "r", serialize = "relative")]
    R = 2,
}

impl LPR {
    pub fn others(&self) -> Vec<Self> {
        Self::iter().filter(|x| x != self).collect()
    }
    pub fn transformations() -> Vec<Self> {
        Self::iter().collect()
    }
}

impl Dirac<Triad> for LPR {
    type Output = Triad;

    fn dirac(&self, triad: &mut Triad) -> Self::Output {
        use ChordFactor::*;
        match triad.intervals().0 {
            Thirds::Major => match *self {
                LPR::L => triad[Root] -= Interval::Semitone,
                LPR::P => triad[Third] -= Interval::Semitone,
                LPR::R => triad[Fifth] += Interval::Tone,
            },
            Thirds::Minor => match *self {
                LPR::L => triad[Fifth] += Interval::Semitone,
                LPR::P => triad[Third] += Interval::Semitone,
                LPR::R => triad[Root] -= Interval::Tone,
            },
        };

        triad.update().expect("Invalid triad")
    }
}

impl std::ops::Mul<Triad> for LPR {
    type Output = Triad;

    fn mul(self, rhs: Triad) -> Self::Output {
        self.dirac(&mut rhs.clone())
    }
}
