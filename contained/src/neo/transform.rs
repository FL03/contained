/*
    Appellation: transform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Shift by a semitone : +/- 1
        Shift by a tone: +/- 2
*/
use crate::neo::{
    cmp::{is_minor_third, Note},
    Triad,
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

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
    L = 0, // Preserves the minor third; shifts the remaining note by a semitone
    P = 1, // Preserves the perfect fifth; shifts the remaining note by a semitone
    R = 2, // preserves the major third in the triad and moves the remaining note by whole tone.
}

impl LPR {
    pub fn transform(&self, triad: &Triad) -> Triad {
        let (mut r, mut t, mut f): (i64, i64, i64) = triad.clone().into();

        let rt_interval = is_minor_third(r.clone(), t.clone());

        match self {
            LPR::L => {
                if rt_interval {
                    f += 1;
                } else {
                    r -= 1;
                }
            }
            LPR::P => {
                if rt_interval {
                    t += 1;
                } else {
                    t -= 1;
                }
            }
            LPR::R => {
                if rt_interval {
                    r -= 2;
                } else {
                    f += 2;
                }
            }
        }
        if r < 0 {
            r += 12;
        }
        if t < 0 {
            r += 12;
        }
        if f < 0 {
            f += 12;
        }
        println!("{:?}", (r, t, f));
        Triad::try_from((r, t, f)).unwrap()
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
    fn test_lpr_transformation() {
        let a = Triad::build(0.into(), Triads::Major);
        let b = LPR::default() * a.clone();
        let c = LPR::L * b.clone();
        assert_ne!(a.clone(), b.clone());
        assert_eq!(b.clone(), Triad::try_from((4, 7, 11)).unwrap());
        assert_eq!(a.clone(), c.clone());
    }
}
