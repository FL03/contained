/*
    Appellation: config <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::turing::Symbolic;
use crate::States;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Configuration<S: Symbolic> {
    index: usize,
    pub state: States,
    tape: Vec<S>,
}

impl<S: Symbolic> Configuration<S> {
    pub fn new(index: usize, state: States, tape: Vec<S>) -> Result<Self, String> {
        if index > tape.len() {
            return Err(format!("The starting position ({}) is out of bounds", index))
        }
        Ok(Self { index, state, tape })
    }
    pub fn norm(tape: Vec<S>) -> Result<Self, String> {
        Self::new(0, 1.into(), tape)
    }
    pub fn std(tape: Vec<S>) -> Result<Self, String> {
        Self::new(tape.len() - 1, 1.into(), tape)
    }
    pub fn cells(&self) -> usize {
        self.tape.len()
    }
}

impl<S: Symbolic> From<Configuration<S>> for usize {
    fn from(d: Configuration<S>) -> usize {
        d.index
    }
}

impl<S: Symbolic> From<Configuration<S>> for States {
    fn from(d: Configuration<S>) -> States {
        d.state
    }
}

impl<S: Symbolic> From<Configuration<S>> for Vec<S> {
    fn from(d: Configuration<S>) -> Vec<S> {
        d.tape
    }
}

impl<S: Symbolic> From<Configuration<S>> for (usize, States, Vec<S>) {
    fn from(d: Configuration<S>) -> Self {
        (d.index, d.state, d.tape)
    }
}

impl<S: Symbolic> TryFrom<(usize, States, Vec<S>)> for Configuration<S> {
    type Error = String;

    fn try_from(d: (usize, States, Vec<S>)) -> Result<Self, Self::Error> {
        Self::new(d.0, d.1, d.2)
    }
}
