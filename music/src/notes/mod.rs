/*
    Appellation: notes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A note is a symbolic representation of a particular pitch; also called a pitch class

*/
pub use self::{note::*, pitch::*};

pub mod classes;
pub(crate) mod note;
pub(crate) mod pitch;

use crate::Gradient;
use classes::NaturalNote;

/// [detect_accidentals] is a function for quickly determining the 'accidental' variations of the natural note
/// Given a [NaturalNote] find its optional sharp and flat variations
pub fn detect_accidentals(natural: NaturalNote) -> (Pitch, Option<Pitch>, Option<Pitch>) {
    let note = natural as i64;
    // Calculate the modulus of the next (a) and prev (b) position
    let ab = ((note + 1).pitch(), (note.pitch() - 1).pitch());

    if NaturalNote::try_from(ab.0).is_ok() {
        // If a natural note exists with a modulus a semitone above the entry, than it only has one option at -1 (flat)
        (note.into(), None, Some(ab.1.into()))
    } else if NaturalNote::try_from(ab.1).is_ok() {
        // If a natural note exists with a modulus a semitone below the entry, than it only has one option at +1 (sharp)
        (note.into(), Some(ab.0.into()), None)
    } else {
        // If a natural note doesn't exists a semitone above or below the entry, than it has two possible variations
        // a sharp a semitone above and a flat a semitone below
        (note.into(), Some(ab.0.into()), Some(ab.1.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect() {
        assert!(NaturalNote::try_from(1).is_err());
        assert_eq!(
            detect_accidentals(NaturalNote::A),
            (Pitch::new(9), Some(Pitch::new(10)), Some(Pitch::new(8)))
        );
        assert_eq!(
            detect_accidentals(NaturalNote::C),
            (Pitch::new(0), Some(Pitch::new(1)), None)
        );
    }
}