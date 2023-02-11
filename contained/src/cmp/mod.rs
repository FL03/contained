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

pub trait Notable: Clone + Default {

    fn pitch(&self) -> Pitch;
}

impl Notable for i64 {

    fn pitch(&self) -> Pitch {
        self.clone().into()
    }
}