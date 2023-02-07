/*
    Appellation: transform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Shift by a semitone : +/- 1
        Shift by a tone: +/- 2
*/
use crate::neo::Triad;
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
    L = 0, // Preservese the third; shift the fifth by a semitone
    P = 1, // Preserves the fifth; shifts the third by a semitone
    R = 2, // Preserves the major third; shifts the fifth a whole note
}

impl LPR {
    pub fn transform(&self, triad: &mut Triad) -> Triad {
        let (r, mut t, mut f): (i64, i64, i64) = triad.clone().into();
        match self.clone() as i64 {
            0 => {
                f += 1;
            }
            1 => {
                t += 1;
            }
            2 => {
                f += 2;
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
        let b = LPR::default().transform(&mut a.clone());
        let c = LPR::default() * a.clone();
        assert_ne!(a.clone(), b.clone());
        assert_eq!(b.clone(), Triad::from((0, 1, 3)));
        assert_eq!(b.clone(), c)
    }
}
