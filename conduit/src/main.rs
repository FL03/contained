/*
    Appellation: Conduit <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{actors::*, components::*, core::*, data::*};
use serde::{Deserialize, Serialize};

pub(crate) mod actors;
pub(crate) mod components;
pub(crate) mod core;
pub(crate) mod data;

fn main() {
    println!("Hello, world!");
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Machine<T> {
    pub inputs: Vec<T>,
    pub outputs: Vec<String>,
}

impl<T> Machine<T> {
    pub fn new(inputs: Vec<T>) -> Self {
        Self {
            inputs,
            outputs: Default::default(),
        }
    }
}
