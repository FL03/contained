/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Triad;
use contained_core::prelude::State;
use std::sync::{Arc, Mutex};

/// [Triadic] is a trait describing the contextual requirements of a [Triad].
pub trait Triadic: Send + Sync {
    type Env;
    type Store;
    
    fn env(&self) -> Self::Env;
    fn state(&self) -> State;
    fn store(&self) -> Self::Store;
    fn triad(&self) -> Triad;
}

pub struct TriadContext<T> {
    
    state: State,
    store: T,
}