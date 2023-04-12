/*
    Appellation: chords <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::chord::*;

mod chord;

pub trait IntoChord {
    fn into_chord(self) -> Chord;
}

impl<T> IntoChord for T
where
    T: Into<Chord>,
{
    fn into_chord(self) -> Chord {
        self.into()
    }
}

pub trait FromChord {
    fn from_chord(chord: Chord) -> Self;
}

impl<T> FromChord for T
where
    T: From<Chord>,
{
    fn from_chord(chord: Chord) -> Self {
        Self::from(chord)
    }
}
