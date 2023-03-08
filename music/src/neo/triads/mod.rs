/*
    Appellation: triads <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A triad is a set of three notes, called chord factors: root, third, and fifth
        Generaically, triad's share two of its notes with three of its inversions.

        For our purposes, a triad is said to be a three-tuple (a, b, c) where both [a, b] and [b, c] are thirds.
*/
pub use self::triad::*;

pub(crate) mod triad;

use crate::{
    intervals::{Fifths, Thirds},
    Notable,
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
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum Triads {
    Augmented, // If the root -> third is major and if third -> fifth is major
    Diminshed, // If the root -> third is minor and if third -> fifth is minor
    #[default]
    Major, // If the root -> third is major and if third -> fifth is minor
    Minor,     // If the root -> third is minor and if third -> fifth is major
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
                Fifths::Diminished => Ok(Self::Diminshed),
                Fifths::Perfect => Ok(Self::Minor),
                _ => Err("".into()),
            },
        }
    }
}
