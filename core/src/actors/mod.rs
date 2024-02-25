/*
    Appellation: actors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Actors
pub use self::actor::*;

pub(crate) mod actor;

pub trait Executor {
    fn execute(&self);
}
