/*
    Appellation: Conduit <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use serde::{Deserialize, Serialize};

fn main() {
    println!("Hello, world!");
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Machine {
    pub inputs: String,
    pub outputs: String
}
