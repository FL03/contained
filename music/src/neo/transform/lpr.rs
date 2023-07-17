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
#[strum(serialize_all = "lowercase")]
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
        use Interval::{Semitone, Tone};
        
        match triad.class().intervals().0 {
            Thirds::Major => match *self {
                LPR::L => triad[Root] -= Semitone,
                LPR::P => triad[Third] -= Semitone,
                LPR::R => triad[Fifth] += Tone,
            },
            Thirds::Minor => match *self {
                LPR::L => triad[Fifth] += Semitone,
                LPR::P => triad[Third] += Semitone,
                LPR::R => triad[Root] -= Tone,
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


#[cfg(test)]
mod tests {
    use std::str::FromStr;


    use super::*;
    use crate::neo::triads::*;
    

    #[test]
    fn test_lpr() {
        assert_eq!(LPR::from_str("l"), LPR::from_str("leading"));
    }

    #[test]
    fn test_leading() {
        let triad = Triad::default();
        assert_eq!(triad.clone(), LPR::L * (LPR::L * triad.clone()));
        assert_ne!(triad.clone(), LPR::L * triad);
    }

    #[test]
    fn test_parallel() {
         let triad = Triad::default();
        assert_eq!(triad.clone(), LPR::P * (LPR::P * triad.clone()));
        assert_ne!(triad.clone(), LPR::P * triad);
    }

    #[test]
    fn test_relative() {
         let triad = Triad::default();
        assert_eq!(triad.clone(), LPR::R * (LPR::R * triad.clone()));
        assert_ne!(triad.clone(), LPR::R * triad);
    }
}