/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{state::*, utils::*};

pub(crate) mod state;

pub fn new() -> States {
    States::default()
}

pub(crate) mod utils {}
