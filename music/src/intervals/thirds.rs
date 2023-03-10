/*
    Appellation: thirds <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A collection of common musical intervals
        A musical third can be either be a difference of three (minor) or four (major) semitones
*/
use crate::{Gradient, Notable};
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
        (Self::Major + note.clone(), Self::Minor + note)
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

impl<N: Notable> TryFrom<[N; 2]> for Thirds {
    type Error = String;

    fn try_from(data: [N; 2]) -> Result<Self, Self::Error> {
        Thirds::try_from((data[0].clone(), data[1].clone()))
    }
}

impl<N: Notable> std::ops::Add<N> for Thirds {
    type Output = N;

    fn add(self, rhs: N) -> Self::Output {
        (rhs.pitch() + self as i64).into()
    }
}

impl<N: Notable> std::ops::Sub<N> for Thirds {
    type Output = N;

    fn sub(self, rhs: N) -> Self::Output {
        (rhs.pitch() - self as i64).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Note;

    #[test]
    fn test_thirds() {
        assert_eq!(Thirds::Major + Note::from(0), Note::from(4))
    }
}
