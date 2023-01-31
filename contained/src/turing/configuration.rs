/*
    Appellation: config <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::turing::{Symbolic, Tape};
use crate::States;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Configuration<S: Symbolic> {
    index: usize,
    pub state: States,
    tape: Tape<S>,
}

impl<S: Symbolic> Configuration<S> {
    pub fn new(index: usize, state: States, tape: Tape<S>) -> Result<Self, String> {
        if index > tape.len() {
            return Err(format!(
                "The starting position ({}) is out of bounds",
                index
            ));
        }
        Ok(Self { index, state, tape })
    }
    pub fn norm(tape: Tape<S>) -> Result<Self, String> {
        Self::new(0, 1.into(), tape)
    }
    pub fn std(tape: Tape<S>) -> Result<Self, String> {
        Self::new(tape.len() - 1, 1.into(), tape)
    }
    pub fn cells(&self) -> usize {
        self.tape().len()
    }
    pub fn position(&self) -> usize {
        self.index
    }
    pub fn state(&self) -> States {
        self.state
    }
    pub fn tape(&self) -> Tape<S> {
        self.tape.clone()
    }
}

impl<S: Symbolic> From<Configuration<S>> for usize {
    fn from(d: Configuration<S>) -> usize {
        d.position()
    }
}

impl<S: Symbolic> From<Configuration<S>> for States {
    fn from(d: Configuration<S>) -> States {
        d.state()
    }
}

impl<S: Symbolic> From<Configuration<S>> for Tape<S> {
    fn from(d: Configuration<S>) -> Tape<S> {
        d.tape()
    }
}

impl<S: Symbolic> From<Configuration<S>> for (usize, States, Tape<S>) {
    fn from(d: Configuration<S>) -> Self {
        (d.index, d.state, d.tape)
    }
}

impl<S: Symbolic> TryFrom<(usize, States, Tape<S>)> for Configuration<S> {
    type Error = String;

    fn try_from(d: (usize, States, Tape<S>)) -> Result<Self, Self::Error> {
        Self::new(d.0, d.1, d.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configuration() {
        let tape = Tape::new(["a", "b", "c"]);
        let a = Configuration::norm(tape.clone());
        let b = Configuration::std(tape);
        assert!(a.is_ok());
        assert!(b.is_ok());
        assert_ne!(a.unwrap(), b.unwrap())
    }
}
