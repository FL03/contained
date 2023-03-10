/*
    Appellation: triads <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A triad is a set of three notes, called chord factors: root, third, and fifth
        Generaically, triad's share two of its notes with three of its inversions.

        For our purposes, a triad is said to be a three-tuple (a, b, c) where the intervals [a, b] and [b, c] are both thirds.
*/
pub use self::triad::*;

pub(crate) mod triad;

use crate::{
    intervals::{Fifths, Thirds},
    Notable,
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

/// [Triads::Augmented] is a [Triad] created with [Thirds::Major], [Thirds::Major] intervals
/// [Triads::Diminished] is a [Triad] created with [Thirds::Minor], [Thirds::Minor] intervals
/// [Triads::Major] is a [Triad] created with [Thirds::Major], [Thirds::Minor] intervals
/// [Triads::Minor] is a [Triad] created with [Thirds::Minor], [Thirds::Major] intervals
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
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum Triads {
    Augmented,
    Diminished,
    #[default]
    Major,
    Minor,
}

impl From<Triads> for (Thirds, Thirds) {
    fn from(class: Triads) -> (Thirds, Thirds) {
        match class {
            Triads::Augmented => (Thirds::Major, Thirds::Major),
            Triads::Diminished => (Thirds::Minor, Thirds::Minor),
            Triads::Major => (Thirds::Major, Thirds::Minor),
            Triads::Minor => (Thirds::Minor, Thirds::Major),
        }
    }
}

impl From<Triads> for (Thirds, Fifths) {
    fn from(class: Triads) -> (Thirds, Fifths) {
        match class {
            Triads::Augmented => (Thirds::Major, Fifths::Augmented),
            Triads::Diminished => (Thirds::Minor, Fifths::Diminished),
            Triads::Major => (Thirds::Major, Fifths::Perfect),
            Triads::Minor => (Thirds::Minor, Fifths::Perfect),
        }
    }
}

impl<N: Notable> TryFrom<Triad<N>> for Triads {
    type Error = Box<dyn std::error::Error>;

    fn try_from(triad: Triad<N>) -> Result<Self, Self::Error> {
        let (r, t, f): (N, N, N) = triad.into();
        let ab = Thirds::try_from((r.clone(), t))?;
        let bc = Fifths::try_from((r, f))?;

        match ab {
            Thirds::Major => match bc {
                Fifths::Augmented => Ok(Self::Augmented),
                Fifths::Perfect => Ok(Self::Major),
                _ => Err("".into()),
            },
            Thirds::Minor => match bc {
                Fifths::Diminished => Ok(Self::Diminished),
                Fifths::Perfect => Ok(Self::Minor),
                _ => Err("".into()),
            },
        }
    }
}
