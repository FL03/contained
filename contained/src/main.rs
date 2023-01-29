/*
    Appellation: Contained <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{states::*, turing::*};

pub(crate) mod states;
pub(crate) mod turing;

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
    pub state: State,
}

impl Machine {
    pub fn new(state: State) -> Self {
        Self { state }
    }
}


