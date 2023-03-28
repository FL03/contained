/*
    Appellation: reqres <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

pub use self::{command::*, event::*};

mod command;
mod event;

pub enum Operation {
    Add,
    Get,
    Remove,
    Run,
    Update,
}
