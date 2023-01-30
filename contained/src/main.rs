/*
    Appellation: Contained <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::states::*;

pub(crate) mod states;

pub mod turing;

use scsys::prelude::AsyncResult;
use std::sync::Arc;

fn main() -> AsyncResult {
    Ok(())
}

pub trait TuringMachine {
    type Alphabet: Clone;
    type State: Default;

    fn args(&self) -> Vec<Self::Alphabet>;
    fn state(&self) -> Self::State;
    fn transition(&self) -> Self::State;
}

pub struct Machine {
    pub state: States,
}

impl Machine {
    pub fn new(state: States) -> Self {
        Self { state }
    }
}
