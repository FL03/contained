/*
    Appellation: machine <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{Program, Symbolic};
use crate::State;
use std::sync::Arc;


pub struct Machine<S: Symbolic> {

    pub program: Program<S>
}

impl<S: Symbolic> Machine<S> {
    pub fn new(program: Program<S>) -> Self {
        Self { program }
    }
}

