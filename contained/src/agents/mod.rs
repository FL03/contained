/*
    Appellation: agents <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: An agent describes a persistent, stateful, and isolated virtual machine.
*/
pub use self::agent::*;

mod agent;

pub mod layer;
pub mod tonic;


pub trait Actor {
    
}