/*
    Appellation: cmp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A collection of musical objects abstracted for computational purposes
*/
pub use self::{chord::*, clef::*, epoch::*, intervals::*, notes::*, pitch::*};

pub(crate) mod chord;
pub(crate) mod clef;
pub(crate) mod epoch;
pub(crate) mod intervals;
pub(crate) mod notes;
pub(crate) mod pitch;

/// [Notable] is used to designate a structure used to represent a note
pub trait Notable: Clone + Default + ToString {
    fn class(&self) -> PitchClass {
        self.pitch().into()
    }
    fn pitch(&self) -> Pitch;
    fn symbol(&self) -> String {
        self.to_string()
    }
}

impl Notable for Pitch {
    fn pitch(&self) -> Pitch {
        self.clone()
    }
}

impl Notable for i64 {
    fn pitch(&self) -> Pitch {
        self.clone().into()
    }
}
