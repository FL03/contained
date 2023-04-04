/*
    Appellation: agents <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: An agent describes a persistent, stateful, and isolated virtual machine.
*/
pub use self::{agent::*, context::*, environment::*, settings::*};

mod agent;
mod context;
mod environment;
mod settings;

pub mod client;
pub mod layer;
pub mod tonic;

use decanter::prelude::H256;
use std::collections::HashMap;
use wasmer::Module;

pub trait Actor {}

pub struct Stack {
    pub modules: HashMap<H256, Module>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }
}
