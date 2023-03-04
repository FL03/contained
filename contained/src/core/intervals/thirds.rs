/*
    Appellation: intervals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A collection of common musical intervals

        Thirds: Major / Minor
        Fifth: Augmented, Dimenished, Perfect
*/
use crate::core::{Gradient, Notable};
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
pub enum Thirds {
    #[default]
    Major = 4,
    Minor = 3,
}

impl Thirds {
    /// [is_third] compares two notes to see if either a major or minor third interval exists
    pub fn is_third<N: Notable>(a: N, b: N) -> Result<Self, String> {
        Self::try_from((a, b))
    }
    pub fn compute<N: Notable>(note: N) -> (N, N) {
        (Self::Major * note.clone(), Self::Minor * note)
    }
    /// Functional method for creating a major third
    pub fn major() -> Self {
        Self::Major
    }
    /// Functional method for creating a minor third
    pub fn minor() -> Self {
        Self::Minor
    }
}

impl<N: Notable> TryFrom<(N, N)> for Thirds {
    type Error = String;

    fn try_from(data: (N, N)) -> Result<Self, Self::Error> {
        // An interval is the difference in pitch between an two notes
        // We take the pitch of the result to account for its modularity; (0, 11) -> 11 but (11, 0) -> 1
        let interval: i64 = (data.1.pitch() - data.0.pitch()).pitch();
        match interval {
            3 => Ok(Self::Minor),
            4 => Ok(Self::Major),
            _ => Err("Interval is not a third...".to_string()),
        }
    }
}

impl<N: Notable> std::ops::Mul<N> for Thirds {
    type Output = N;

    fn mul(self, rhs: N) -> Self::Output {
        (rhs.pitch() + self.clone() as i64).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Note;

    #[test]
    fn test_thirds() {
        let a: Note = 0.into();
        let b = Thirds::Major * a;
        assert_eq!(b, Note::from(4))
    }
}
