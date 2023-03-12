/*
    Appellation: compute <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: this module implements a computable surface created with a triad
*/
pub use self::{quadrant::*, surface::*};

pub(crate) mod quadrant;
pub(crate) mod surface;