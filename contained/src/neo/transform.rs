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
use super::{Triad, Triadic};
use crate::{
    absmod,
    cmp::{is_minor_third, Gradient, Notable},
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

pub fn leading<N: Notable>(triad: &Triad<N>) -> Triad<N> {
    let (mut r, t, mut f): (i64, i64, i64) = (
        triad.root().pitch(),
        triad.third().pitch(),
        triad.fifth().pitch(),
    );
    if is_minor_third(triad.root(), triad.third()) {
        f += 1;
    } else {
        r -= 1;
    }
    let triad = (r.pitch(), t.pitch(), f.pitch());
    // All triadic transformations will result in another valid triad
    Triad::try_from(triad).unwrap()
}

pub fn parallel<N: Notable>(triad: &Triad<N>) -> Triad<N> {
    let (r, mut t, f): (i64, i64, i64) = (
        triad.root().pitch(),
        triad.third().pitch(),
        triad.fifth().pitch(),
    );
    if is_minor_third(triad.root(), triad.third()) {
        t += 1;
    } else {
        t -= 1;
    }
    let triad = (r.pitch(), t.pitch(), f.pitch());
    // All triadic transformations will result in another valid triad
    Triad::try_from(triad).unwrap()
}

pub fn relative<N: Notable>(triad: &Triad<N>) -> Triad<N> {
    let (mut r, t, mut f): (i64, i64, i64) = (
        triad.root().pitch(),
        triad.third().pitch(),
        triad.fifth().pitch(),
    );
    if is_minor_third(triad.root(), triad.third()) {
        r -= 2;
    } else {
        f += 2;
    }
    let triad = (r.pitch(), t.pitch(), f.pitch());
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
    pub fn transform<N: Notable>(&self, triad: &Triad<N>) -> Triad<N> {
        match self {
            LPR::L => leading(triad),
            LPR::P => parallel(triad),
            LPR::R => relative(triad),
        }
    }
}

impl<N: Notable> std::ops::Mul<Triad<N>> for LPR {
    type Output = Triad<N>;

    fn mul(self, rhs: Triad<N>) -> Self::Output {
        self.transform(&mut rhs.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmp::Note;
    use crate::neo::{Triad, Triads};

    #[test]
    fn test_leading() {
        let a = Triad::<Note>::new(0.into(), Triads::Major);
        let b = LPR::L * a.clone();
        assert_ne!(a, b);
        assert_eq!(b, Triad::<Note>::try_from((4, 7, 11)).unwrap());
        assert_eq!(a, LPR::L * b);
    }

    #[test]
    fn test_parallel() {
        let a = Triad::<Note>::new(0.into(), Triads::Major);
        let b = LPR::P * a.clone();
        assert_ne!(a, b);
        assert_eq!(b, Triad::<Note>::try_from((0, 3, 7)).unwrap());
        assert_eq!(LPR::P * b, a)
    }

    #[test]
    fn test_relative() {
        let a = Triad::<Note>::new(0.into(), Triads::Major);
        let b = LPR::R * a.clone();

        assert_ne!(a, b);
        assert_eq!(b, Triad::<Note>::try_from((9, 0, 4)).unwrap());
        assert_eq!(LPR::R * b, a)
    }
}
