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

use crate::Gradient;

/// [detect_accidentals] is a function for quickly determining the 'accidental' variations of the natural note
/// Given a [NaturalNote] find its optional sharp and flat variations
pub fn detect_accidentals(natural: NaturalNote) -> (i64, Option<i64>, Option<i64>) {
    let note = natural as i64;
    // Calculate the modulus of the next (a) and prev (b) position
    let ab = ((note + 1).pitch(), (note.pitch() - 1).pitch());

    if NaturalNote::try_from(ab.0).is_ok() {
        // If a natural note exists with a modulus a semitone above the entry, than it only has one option at -1 (flat)
        (note, None, Some(ab.1))
    } else if NaturalNote::try_from(ab.1).is_ok() {
        // If a natural note exists with a modulus a semitone below the entry, than it only has one option at +1 (sharp)
        (note, Some(ab.0), None)
    } else {
        // If a natural note doesn't exists a semitone above or below the entry, than it has two possible variations
        // a sharp a semitone above and a flat a semitone below
        (note, Some(ab.0), Some(ab.1))
    }
}

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
