/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Triad;
use contained_core::prelude::State;
use std::sync::{Arc, Mutex};

/// [Triadic] is a trait describing the contextual requirements of a [Triad].
pub trait Triadic: Send + Sync {
    type Store;
    
    fn state(&self) -> State;
    fn store(&self) -> Self::Store;
    fn triad(&self) -> Triad;
}

pub struct TriadContext<T> {
    state: State,
    store: T,
    triad: Triad,
}

impl<T> TriadContext<T> {
    pub fn new(state: State, store: T, triad: Triad) -> Self {
        Self {
            state,
            store,
            triad,
        }
    }
    
}