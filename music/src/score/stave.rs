/*
    Appellation: stave <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A stave is typically composed of five lines and four spaces, and is used to represent the staff on which notes are written.
*/
use super::Clef;
use crate::chords::Chord;

pub struct Stave {
    clef: Clef,
    measures: Vec<Chord>,
}
