/*
    Appellation: neo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        This module is dedicated to the proposed harmonic computational fabric
*/
pub use self::{link::*, tonnetz::*, transform::*};

pub(crate) mod link;
pub(crate) mod tonnetz;
pub(crate) mod transform;

pub mod compute;
pub mod triads;
