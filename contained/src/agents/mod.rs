/*
    Appellation: agents <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: An agent describes a persistent, stateful, and isolated virtual machine.
*/
pub use self::{agent::*, environment::*, stack::*};

mod agent;
mod environment;
mod stack;

pub mod client;
pub mod layer;

pub trait Actor {}
