/*
    Appellation: cmp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A collection of musical objects abstracted for computational purposes
*/
pub use self::{chord::*, epoch::*, note::*, pitch::*};

pub(crate) mod chord;
pub(crate) mod epoch;
pub(crate) mod note;
pub(crate) mod pitch;
