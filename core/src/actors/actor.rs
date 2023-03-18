/*
    Appellation: actor <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::states::State;
use crate::Symbolic;

pub struct Actor<S: Symbolic> {
    index: usize,
    inputs: Vec<S>,
    outputs: Vec<S>,
    state: State,
}

impl<S: Symbolic> Actor<S> {}
