/*
    Appellation: intervals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A collection of common musical intervals
*/
pub use self::{fifths::*, thirds::*};

pub(crate) mod fifths;
pub(crate) mod thirds;

use crate::{Gradient, Notable};

use decanter::prelude::{hasher, Hashable, H256};
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
    Semitone = 1,
    Tone = 2,
    Fifth(Fifths),
    #[default]
    Third(Thirds),
}

impl Hashable for Interval {
    fn hash(&self) -> H256 {
        hasher(self).into()
    }
}

impl From<Interval> for i64 {
    fn from(interval: Interval) -> i64 {
        match interval {
            Interval::Fifth(i) => i as i64,
            Interval::Third(i) => i as i64,
            Interval::Semitone => 1,
            Interval::Tone => 2,
        }
    }
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

impl<N: Notable> std::ops::Add<N> for Interval {
    type Output = N;

    fn add(self, rhs: N) -> Self::Output {
        let interval: i64 = self.into();
        (rhs.pitch() + interval).into()
    }
}

impl<N: Notable> std::ops::Sub<N> for Interval {
    type Output = N;

    fn sub(self, rhs: N) -> Self::Output {
        let interval: i64 = self.into();
        (rhs.pitch() - interval).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Note;

    #[test]
    fn test_interval() {
        assert_eq!(Interval::from(Thirds::Major) + Note::from(0), Note::from(4))
    }
}
