/*
    Appellation: notes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A note is a symbolic representation of a particular pitch; also called a pitch class

*/
pub use self::{accidentals::*, naturals::*, note::*};

pub(crate) mod accidentals;
pub(crate) mod naturals;
pub(crate) mod note;

use super::{Gradient, Pitch, PitchClass};

/// [detect_accidentals] is a function for quickly determining the 'accidental' variations of the natural note
/// Given a [NaturalNote] find its optional sharp and flat variations
pub fn detect_accidentals(natural: NaturalNote) -> (i64, Option<i64>, Option<i64>) {
    let note = natural as i64;
    // Calculate the modulus of the next (a) and prev (b) position
    let (a, b) = if note == 0 {
        (1, 11)
    } else {
        ((note + 1) % 12, (note - 1) % 12)
    };
    // If a natural note exists with a modulus a semitone above the entry, than it only has one option at -1 (flat)
    if NaturalNote::try_from(a).is_ok() {
        return (note, None, Some(b));
    }
    // If a natural note exists with a modulus a semitone below the entry, than it only has one option at +1 (sharp)
    if NaturalNote::try_from(b).is_ok() {
        return (note, Some(a), None);
    }
    // If a natural note doesn't exists a semitone above or below the entry, than it has two possible variations
    // a sharp a semitone above and a flat a semitone below
    (note, Some(a), Some(b))
}

/// [Notable] is used to designate a structure used to represent a note
pub trait Notable: Clone + Default + Gradient + ToString {
    fn class(&self) -> PitchClass {
        self.pitch().into()
    }
}

impl Notable for Pitch {}

impl Notable for i64 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect() {
        assert!(NaturalNote::try_from(1).is_err());
        assert_eq!(detect_accidentals(NaturalNote::A), (9, Some(10), Some(8)));
        assert_eq!(detect_accidentals(NaturalNote::C), (0, Some(1), None));
    }
}
