/*
    Appellation: score <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A score is a collection of staves, which are collections of measures, which are collections of notes.
*/
pub use self::{clef::*, measure::*, stave::*};

mod clef;
mod measure;
mod stave;
