/*
    Appellation: class <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::Triad;
use crate::{
    intervals::{Fifths, Interval, Thirds},
    BoxedError, Note,
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

impl Triads {
    pub fn intervals(&self) -> (Thirds, Thirds, Fifths) {
        match self {
            Triads::Augmented => (Thirds::Major, Thirds::Major, Fifths::Augmented),
            Triads::Diminished => (Thirds::Minor, Thirds::Minor, Fifths::Diminished),
            Triads::Major => (Thirds::Major, Thirds::Minor, Fifths::Perfect),
            Triads::Minor => (Thirds::Minor, Thirds::Major, Fifths::Perfect),
        }
    }
}

impl From<(Thirds, Thirds)> for Triads {
    fn from(intervals: (Thirds, Thirds)) -> Triads {
        match intervals.0 {
            Thirds::Major => match intervals.1 {
                Thirds::Major => Triads::Augmented,
                Thirds::Minor => Triads::Major,
            },
            Thirds::Minor => match intervals.1 {
                Thirds::Major => Triads::Diminished,
                Thirds::Minor => Triads::Minor,
            },
        }
    }
}

impl TryFrom<Triad> for Triads {
    type Error = BoxedError;

    fn try_from(data: Triad) -> Result<Self, Self::Error> {
        let triad: (Note, Note, Note) = data.into();
        Self::try_from(triad)
    }
}

impl TryFrom<(Thirds, Fifths)> for Triads {
    type Error = BoxedError;

    fn try_from(intervals: (Thirds, Fifths)) -> Result<Triads, Self::Error> {
        match intervals.0 {
            Thirds::Major => match intervals.1 {
                Fifths::Augmented => Ok(Triads::Augmented),
                Fifths::Diminished => {
                    Err("Cannot create a triad with a major third and a diminished fifth".into())
                }
                Fifths::Perfect => Ok(Triads::Major),
            },
            Thirds::Minor => match intervals.1 {
                Fifths::Augmented => Err(
                    "Cannot create an augmented triad with a minor third and an augmented fifth"
                        .into(),
                ),
                Fifths::Diminished => Ok(Triads::Diminished),
                Fifths::Perfect => Ok(Triads::Minor),
            },
        }
    }
}

impl TryFrom<(Note, Note, Note)> for Triads {
    type Error = BoxedError;

    fn try_from(data: (Note, Note, Note)) -> Result<Self, Self::Error> {
        let (r, t, f): (Note, Note, Note) = (data.0, data.1, data.2);
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

impl From<Triads> for (Interval, Interval, Interval) {
    fn from(class: Triads) -> (Interval, Interval, Interval) {
        let intervals: (Thirds, Thirds, Fifths) = class.into();
        (intervals.0.into(), intervals.1.into(), intervals.2.into())
    }
}

impl From<Triads> for (Thirds, Thirds, Fifths) {
    fn from(class: Triads) -> (Thirds, Thirds, Fifths) {
        match class {
            Triads::Augmented => (Thirds::Major, Thirds::Major, Fifths::Augmented),
            Triads::Diminished => (Thirds::Minor, Thirds::Minor, Fifths::Diminished),
            Triads::Major => (Thirds::Major, Thirds::Minor, Fifths::Perfect),
            Triads::Minor => (Thirds::Minor, Thirds::Major, Fifths::Perfect),
        }
    }
}
