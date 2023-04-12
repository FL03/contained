/*
    Appellation: notes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A note is a symbolic representation of a particular pitch; also called a pitch class

*/
pub use self::{aspn::*, classes::*, note::*, pitch::*};

mod aspn;
mod classes;
mod note;
mod pitch;

use crate::Gradient;

/// [detect_accidentals] is a function for quickly determining the 'accidental' variations of the natural note
/// Given a [NaturalNote] find its optional sharp and flat variations
pub fn detect_accidentals(natural: Naturals) -> (Pitch, Option<Pitch>, Option<Pitch>) {
    let note = natural as i64;
    // Calculate the modulus of the next (a) and prev (b) position
    let ab = ((note + 1).pitch(), (note.pitch() - 1).pitch());

    if Naturals::try_from(ab.0).is_ok() {
        // If a natural note exists with a modulus a semitone above the entry, than it only has one option at -1 (flat)
        (note.into(), None, Some(ab.1.into()))
    } else if Naturals::try_from(ab.1).is_ok() {
        // If a natural note exists with a modulus a semitone below the entry, than it only has one option at +1 (sharp)
        (note.into(), Some(ab.0.into()), None)
    } else {
        // If a natural note doesn't exists a semitone above or below the entry, than it has two possible variations
        // a sharp a semitone above and a flat a semitone below
        (note.into(), Some(ab.0.into()), Some(ab.1.into()))
    }
}

/// [IntoPitch] describes the conversion of a given object into a [Pitch]
pub trait IntoPitch {
    fn into_pitch(self) -> Pitch;
}

impl<T> IntoPitch for T
where
    T: Into<Pitch>,
{
    fn into_pitch(self) -> Pitch {
        self.into()
    }
}

/// [FromPitch] describes the conversion of a [Pitch] into a given object
pub trait FromPitch {
    fn from_pitch(pitch: Pitch) -> Self;
}

impl<T> FromPitch for T
where
    T: From<Pitch>,
{
    fn from_pitch(pitch: Pitch) -> Self {
        Self::from(pitch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect() {
        assert!(Naturals::try_from(1).is_err());
        assert_eq!(
            detect_accidentals(Naturals::A),
            (Pitch::new(9), Some(Pitch::new(10)), Some(Pitch::new(8)))
        );
        assert_eq!(
            detect_accidentals(Naturals::C),
            (Pitch::new(0), Some(Pitch::new(1)), None)
        );
    }
}
