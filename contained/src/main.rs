/*
    Appellation: Contained <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::states::*;

pub(crate) mod states;

pub mod turing;

use scsys::prelude::AsyncResult;

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


