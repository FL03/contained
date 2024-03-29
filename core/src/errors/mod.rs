/*
    Appellation: errors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{asynchronous::*, error::*};

mod asynchronous;
mod error;

pub trait BaseError: std::error::Error {}
