/*
    Appellation: intervals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A collection of common musical intervals
*/
pub use self::{fifths::*, thirds::*};

pub(crate) mod fifths;
pub(crate) mod thirds;

use crate::music::Notable;
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
pub enum Interval {
    Fifth(Fifths),
    #[default]
    Third(Thirds),
}

impl From<Fifths> for Interval {
    fn from(data: Fifths) -> Interval {
        Interval::Fifth(data)
    }
}

impl From<Thirds> for Interval {
    fn from(data: Thirds) -> Interval {
        Interval::Third(data)
    }
}

impl<N: Notable> std::ops::Mul<N> for Interval {
    type Output = N;

    fn mul(self, rhs: N) -> Self::Output {
        let interval: i64 = match self {
            Interval::Fifth(i) => i as i64,
            Interval::Third(i) => i as i64,
        };
        (rhs.pitch() + interval).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::music::Note;

    #[test]
    fn test_interval() {
        assert_eq!(Interval::from(Thirds::Major) * Note::from(0), Note::from(4))
    }
}
