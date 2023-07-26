/*
    Appellation: agents <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Agents
pub use self::{agent::*, context::*, environment::*, stack::*};

mod agent;
mod context;
mod environment;
mod stack;

pub mod client;
pub mod layer;

pub trait Actor {}
