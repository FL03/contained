/*
    Appellation: transform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Shift by a semitone : +/- 1
        Shift by a tone: +/- 2
*/
use crate::neo::{Triad, is_third};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

use super::is_minor_third;

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

        let (a, b) = (is_minor_third(r.clone(), t.clone()), is_minor_third(t.clone(), f.clone()));
        
        match self.clone() as i64 {
            0 => {
                if !a && b {
                    r -= 1;
                }
                if a && !b {
                    f -= 1;
                }
            }
            1 => {
                if a {
                    t += 1;
                } else {
                    t -= 1;
                }
            }
            2 => {
                if !a && b {
                    f += 1;
                }
                if a && !b {
                    r += 1;
                }
            }
            _ => {}
        }
        Triad::from((r, t, f))
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
    use crate::neo::Triad;

    #[test]
    fn test_lpr_transformation() {
        let a = Triad::from((0, 1, 14));
        let b = LPR::default().transform(&a.clone());
        let c = LPR::default() * a.clone();
        assert_ne!(a.clone(), b.clone());
        assert_eq!(b.clone(), Triad::from((0, 1, 3)));
        assert_eq!(b.clone(), c)
    }
}
