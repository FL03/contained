/*
    Appellation: pitch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        A pitch essentially represents the frequency of a sound wave and has been mathematically expressed to be
            p = log(2f)
        empirically based on the octave doubling of frequency exponetially

        * All notes or pitches are of mod 12, giving us { 0, 1, ..., 10, 11 }
        * Sharp notes and flat notes are simply opposite; if sharp is up then flat is down
            For our purposes, sharp notes are represented with positive integers while flat notes are reserved for negatives
*/
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

/// [detect_accidentals] is a function for quickly determining the 'accidental' variations of the natural note
/// Given a [NaturalNote] find its optional sharp and flat variations
pub fn detect_accidentals(natural: NaturalNote) -> (i64, Option<i64>, Option<i64>) {
    let note = natural as i64;
    // Calculate the modulus of the next (a) and prev (b) position
    let (a, b) = if note.clone() == 0 {
        (1, 11)
    } else {
        ((note.clone() + 1) % 12, (note.clone() - 1) % 12)
    };
    // If a natural note exists with a modulus a semitone above the entry, than it only has one option at -1 (flat)
    if NaturalNote::try_from(a.clone()).is_ok() {
        return (note, None, Some(b));
    }
    // If a natural note exists with a modulus a semitone below the entry, than it only has one option at +1 (sharp)
    if NaturalNote::try_from(b.clone()).is_ok() {
        return (note, Some(a), None);
    }
    // If a natural note doesn't exists a semitone above or below the entry, than it has two possible variations
    // a sharp a semitone above and a flat a semitone below
    (note, Some(a), Some(b))
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
#[strum(serialize_all = "snake_case")]
pub enum Accidentals {
    Flat(FlatNote),
    #[default]
    Sharp(SharpNote),
}

impl TryFrom<i64> for Accidentals {
    type Error = String;

    fn try_from(data: i64) -> Result<Accidentals, Self::Error> {
        if NaturalNote::try_from(data).is_err() {
            let note = if data >= 0 {
                Accidentals::Sharp(SharpNote::try_from(data)?)
            } else {
                Accidentals::Flat(FlatNote::try_from(data)?)
            };
            return Ok(note);
        } else {
            return Err(String::from("Provided note is natural"));
        }
    }
}

impl From<Accidentals> for Pitch {
    fn from(data: Accidentals) -> Pitch {
        let pitch = match data {
            Accidentals::Flat(n) => n as i64,
            Accidentals::Sharp(n) => n as i64,
        };
        Pitch::from(pitch)
    }
}

impl From<FlatNote> for Accidentals {
    fn from(data: FlatNote) -> Accidentals {
        Accidentals::Flat(data)
    }
}

impl From<SharpNote> for Accidentals {
    fn from(data: SharpNote) -> Accidentals {
        Accidentals::Sharp(data)
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
#[strum(serialize_all = "snake_case")]
pub enum PitchClass {
    Accidental(Accidentals),
    #[default]
    Natural(NaturalNote),
}

impl From<Accidentals> for PitchClass {
    fn from(data: Accidentals) -> PitchClass {
        PitchClass::Accidental(data)
    }
}

impl From<NaturalNote> for PitchClass {
    fn from(data: NaturalNote) -> PitchClass {
        PitchClass::Natural(data)
    }
}

impl From<PitchClass> for Pitch {
    fn from(data: PitchClass) -> Pitch {
        match data {
            PitchClass::Accidental(v) => v.into(),
            PitchClass::Natural(n) => Pitch::from(n as i64),
        }
    }
}

impl From<Pitch> for PitchClass {
    fn from(value: Pitch) -> PitchClass {
        PitchClass::from(value.pitch())
    }
}

impl From<i64> for PitchClass {
    fn from(value: i64) -> PitchClass {
        let data = value % 12;
        if let Ok(v) = Accidentals::try_from(data) {
            return PitchClass::from(v);
        } else {
            return PitchClass::from(NaturalNote::try_from(data).expect(""));
        }
    }
}

/// [Pitch] describes the modular index of a given frequency
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Pitch(i64);

impl Pitch {
    pub fn new(pitch: i64) -> Self {
        Self(pitch % 12)
    }
    pub fn pitch(&self) -> i64 {
        self.0
    }
    /// Simple way to detect if the pitch is natural or not
    pub fn is_natural(&self) -> bool {
        NaturalNote::try_from(self.pitch()).is_ok()
    }
}

impl std::fmt::Display for Pitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for Pitch {
    fn from(p: i64) -> Pitch {
        Pitch::new(p)
    }
}

impl From<Pitch> for i64 {
    fn from(p: Pitch) -> i64 {
        p.0
    }
}

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
pub enum FlatNote {
    A = 8,
    B = 10,
    #[default]
    D = 1,
    E = 3,
    G = 6,
}

impl TryFrom<i64> for FlatNote {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let data = value.clone().abs() % 12;
        match data {
            1 => Ok(Self::D),
            3 => Ok(Self::E),
            6 => Ok(Self::G),
            8 => Ok(Self::A),
            10 => Ok(Self::B),
            _ => Err(format!("")),
        }
    }
}

impl TryFrom<Pitch> for FlatNote {
    type Error = String;

    fn try_from(value: Pitch) -> Result<Self, Self::Error> {
        FlatNote::try_from(value.pitch())
    }
}

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
pub enum SharpNote {
    A = 10,
    #[default]
    C = 1,
    D = 3,
    F = 6,
    G = 8,
}

impl TryFrom<Pitch> for SharpNote {
    type Error = String;

    fn try_from(value: Pitch) -> Result<Self, Self::Error> {
        SharpNote::try_from(value.pitch())
    }
}

impl TryFrom<i64> for SharpNote {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let data = value.clone() % 12;
        match data {
            1 => Ok(Self::C),
            3 => Ok(Self::D),
            6 => Ok(Self::F),
            8 => Ok(Self::G),
            10 => Ok(Self::A),
            _ => Err(format!("")),
        }
    }
}

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
pub enum NaturalNote {
    #[default]
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
}

impl TryFrom<Pitch> for NaturalNote {
    type Error = String;

    fn try_from(value: Pitch) -> Result<Self, Self::Error> {
        NaturalNote::try_from(value.pitch())
    }
}

impl TryFrom<i64> for NaturalNote {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let data = value.clone() % 12;
        match data {
            0 => Ok(Self::C),
            2 => Ok(Self::D),
            4 => Ok(Self::E),
            5 => Ok(Self::F),
            7 => Ok(Self::G),
            9 => Ok(Self::A),
            11 => Ok(Self::B),
            _ => Err(format!("")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notes() {
        let a = PitchClass::default();
        let b = PitchClass::Accidental(Accidentals::default());
        assert_ne!(a.clone(), b.clone());
        assert_eq!(a, PitchClass::Natural(Default::default()));
        assert_eq!(
            b,
            PitchClass::Accidental(Accidentals::Sharp(Default::default()))
        )
    }

    #[test]
    fn test_pitch_class() {
        let a = PitchClass::from(-3);
        assert_eq!(
            a.clone(),
            PitchClass::Accidental(Accidentals::Flat(FlatNote::E))
        );
    }

    #[test]
    fn test_detect() {
        assert!(NaturalNote::try_from(1).is_err());
        assert_eq!(detect_accidentals(NaturalNote::A), (9, Some(10), Some(8)));
        assert_eq!(detect_accidentals(NaturalNote::C), (0, Some(1), None));
    }
    #[test]
    fn test_pitch() {
        let a = Pitch::from(144);
        let b = Pitch::from(12);
        assert_eq!(a.clone(), b.clone());
        assert!(a.is_natural())
    }
}
