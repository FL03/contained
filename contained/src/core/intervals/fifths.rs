/*
    Appellation: fifths <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:

        Fifths:
            Augmented (8)
            Perfect (7)
            Diminished (9)
*/
use crate::core::Notable;

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
)]
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum Fifths {
    Augmented = 8,
    Diminished = 6,
    #[default]
    Perfect = 7,
}

impl Fifths {
    pub fn compute<N: Notable>(note: N) -> (N, N, N) {
        (
            Self::Augmented * note.clone(),
            Self::Perfect * note.clone(),
            Self::Diminished * note,
        )
    }
}

impl<N: Notable> std::ops::Mul<N> for Fifths {
    type Output = N;

    fn mul(self, rhs: N) -> Self::Output {
        (rhs.pitch() + self as i64).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Note;

    #[test]
    fn test_fifths() {
        let a: Note = 0.into();
        let b = Fifths::Perfect * a;
        assert_eq!(b, Note::from(7))
    }
}
