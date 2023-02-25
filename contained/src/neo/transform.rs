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
*/
use super::Triad;
use crate::cmp::is_minor_third;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

/// [harmonic_transformation] is a transformative function for continuous musical space
/// This is useful for describing the behavior between transitions as nothing is achieved instantly
pub fn harmonic_transformation(a: usize, b: usize, t: usize) -> usize {
    (b - a) * t + a
}

/// [absmod] is short for the absolute value of a modular number;
fn absmod(a: i64, m: i64) -> i64 {
    ((a + m) % m).abs()
}

pub fn leading(triad: &Triad) -> Triad {
    let (mut r, t, mut f): (i64, i64, i64) = triad.clone().into();
    if is_minor_third(r.clone(), t.clone()) {
        f += 1;
    } else {
        r -= 1;
    }
    let triad = (absmod(r, 12), absmod(t, 12), absmod(f, 12));
    // All triadic transformations will result in another valid triad
    Triad::try_from(triad).unwrap()
}

pub fn parallel(triad: &Triad) -> Triad {
    let (r, mut t, f): (i64, i64, i64) = triad.clone().into();
    if is_minor_third(r.clone(), t.clone()) {
        t += 1;
    } else {
        t -= 1;
    }
    let triad = (absmod(r, 12), absmod(t, 12), absmod(f, 12));
    // All triadic transformations will result in another valid triad
    Triad::try_from(triad).unwrap()
}

pub fn relative(triad: &Triad) -> Triad {
    let (mut r, t, mut f): (i64, i64, i64) = triad.clone().into();
    if is_minor_third(r.clone(), t.clone()) {
        r -= 2;
    } else {
        f += 2;
    }
    let triad = (absmod(r, 12), absmod(t, 12), absmod(f, 12));
    // All triadic transformations will result in another valid triad
    Triad::try_from(triad).unwrap()
}

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
    pub fn transform(&self, triad: &Triad) -> Triad {
        match self {
            LPR::L => leading(triad),
            LPR::P => parallel(triad),
            LPR::R => relative(triad),
        }
    }
}

impl std::ops::Mul<Triad> for LPR {
    type Output = Triad;

    fn mul(self, rhs: Triad) -> Self::Output {
        self.transform(&mut rhs.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neo::{Triad, Triads};

    #[test]
    fn test_leading() {
        let a = Triad::new(0.into(), Triads::Major);
        let b = LPR::L * a.clone();
        assert_ne!(a, b);
        assert_eq!(b, Triad::try_from((4, 7, 11)).unwrap());
        assert_eq!(a, LPR::L * b);
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
        assert_eq!(b, Triad::try_from((0, 4, 9)).unwrap());
        assert_eq!(LPR::R * b, a)
    }
}
