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

/// [TriadClass::Augmented] is a [Triad] created with [Thirds::Major], [Thirds::Major] intervals
/// [TriadClass::Diminished] is a [Triad] created with [Thirds::Minor], [Thirds::Minor] intervals
/// [TriadClass::Major] is a [Triad] created with [Thirds::Major], [Thirds::Minor] intervals
/// [TriadClass::Minor] is a [Triad] created with [Thirds::Minor], [Thirds::Major] intervals
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
pub enum TriadClass {
    Augmented,
    Diminished,
    #[default]
    Major,
    Minor,
}

impl TriadClass {
    pub fn classes() -> Vec<Self> {
        vec![
            TriadClass::Augmented,
            TriadClass::Diminished,
            TriadClass::Major,
            TriadClass::Minor,
        ]
    }
    pub fn others(&self) -> Vec<Self> {
        vec![
            TriadClass::Augmented,
            TriadClass::Diminished,
            TriadClass::Major,
            TriadClass::Minor,
        ]
        .into_iter()
        .filter(|x| x != self)
        .collect()
    }
    pub fn intervals(&self) -> (Thirds, Thirds, Fifths) {
        match self {
            TriadClass::Augmented => (Thirds::Major, Thirds::Major, Fifths::Augmented),
            TriadClass::Diminished => (Thirds::Minor, Thirds::Minor, Fifths::Diminished),
            TriadClass::Major => (Thirds::Major, Thirds::Minor, Fifths::Perfect),
            TriadClass::Minor => (Thirds::Minor, Thirds::Major, Fifths::Perfect),
        }
    }
}

impl From<(Thirds, Thirds)> for TriadClass {
    fn from(intervals: (Thirds, Thirds)) -> TriadClass {
        match intervals.0 {
            Thirds::Major => match intervals.1 {
                Thirds::Major => TriadClass::Augmented,
                Thirds::Minor => TriadClass::Major,
            },
            Thirds::Minor => match intervals.1 {
                Thirds::Major => TriadClass::Minor,
                Thirds::Minor => TriadClass::Diminished,
            },
        }
    }
}

impl TryFrom<Triad> for TriadClass {
    type Error = BoxedError;

    fn try_from(data: Triad) -> Result<Self, Self::Error> {
        let triad: (Note, Note, Note) = data.into();
        Self::try_from(triad)
    }
}

impl TryFrom<(Thirds, Fifths)> for TriadClass {
    type Error = BoxedError;

    fn try_from(intervals: (Thirds, Fifths)) -> Result<TriadClass, Self::Error> {
        match intervals.0 {
            Thirds::Major => match intervals.1 {
                Fifths::Augmented => Ok(TriadClass::Augmented),
                Fifths::Diminished => {
                    Err("Cannot create a triad with a major third and a diminished fifth".into())
                }
                Fifths::Perfect => Ok(TriadClass::Major),
            },
            Thirds::Minor => match intervals.1 {
                Fifths::Augmented => Err(
                    "Cannot create an augmented triad with a minor third and an augmented fifth"
                        .into(),
                ),
                Fifths::Diminished => Ok(TriadClass::Diminished),
                Fifths::Perfect => Ok(TriadClass::Minor),
            },
        }
    }
}

impl TryFrom<[Note; 3]> for TriadClass {
    type Error = BoxedError;

    fn try_from(data: [Note; 3]) -> Result<Self, Self::Error> {
        let [r, t, f]: [Note; 3] = data;
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

impl TryFrom<(Note, Note, Note)> for TriadClass {
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

impl From<TriadClass> for (Interval, Interval, Interval) {
    fn from(class: TriadClass) -> (Interval, Interval, Interval) {
        let intervals: (Thirds, Thirds, Fifths) = class.into();
        (intervals.0.into(), intervals.1.into(), intervals.2.into())
    }
}

impl From<TriadClass> for (Thirds, Thirds, Fifths) {
    fn from(class: TriadClass) -> (Thirds, Thirds, Fifths) {
        match class {
            TriadClass::Augmented => (Thirds::Major, Thirds::Major, Fifths::Augmented),
            TriadClass::Diminished => (Thirds::Minor, Thirds::Minor, Fifths::Diminished),
            TriadClass::Major => (Thirds::Major, Thirds::Minor, Fifths::Perfect),
            TriadClass::Minor => (Thirds::Minor, Thirds::Major, Fifths::Perfect),
        }
    }
}
