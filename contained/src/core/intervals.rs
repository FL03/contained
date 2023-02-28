/*
    Appellation: intervals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A collection of common musical intervals

        Thirds: Major / Minor
        Fifth: Augmented, Dimenished, Perfect
*/
use super::{Gradient, Notable};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

/// [is_third] compares two notes to see if either a major or minor third interval exists
pub fn is_third<N: Notable>(a: N, b: N) -> bool {
    if Thirds::Major * a.clone() == b || Thirds::Minor * a == b {
        return true;
    }
    false
}

/// [is_major_third] compares the given notes and determines if a major third exists
pub fn is_major_third<N: Notable>(a: N, b: N) -> bool {
    if Thirds::Major * a == b {
        return true;
    }
    false
}

/// [is_minor_third]
pub fn is_minor_third<N: Notable>(a: N, b: N) -> bool {
    if Thirds::Minor * a == b {
        return true;
    }
    false
}

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
pub enum Interval {
    Fifth(Fifths),
    #[default]
    Third(Thirds),
}

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
    Diminshed = 6,
    #[default]
    Perfect = 7,
}

impl<N: Notable> std::ops::Mul<N> for Fifths {
    type Output = N;

    fn mul(self, rhs: N) -> Self::Output {
        (rhs.pitch() + self as i64).into()
    }
}

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
    pub fn is_third<N: Notable>(a: N, b: N) -> bool {
        Self::try_from((a, b)).is_ok()
    }
    pub fn compute_both<N: Notable>(note: N) -> (N, N) {
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
