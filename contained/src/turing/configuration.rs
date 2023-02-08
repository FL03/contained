/*
    Appellation: config <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::turing::{Move, Symbolic, Tape};
use crate::State;

use serde::{Deserialize, Serialize};

pub enum Configurations<S: Symbolic> {
    Normal(Configuration<S>),
    Standard(Configuration<S>),
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Configuration<S: Symbolic> {
    index: usize,
    pub(crate) state: State,
    tape: Tape<S>,
}

impl<S: Symbolic> Configuration<S> {
    pub fn new(index: usize, state: State, tape: Tape<S>) -> Result<Self, String> {
        Self::try_from((index, state, tape))
    }
    pub fn norm(tape: Tape<S>) -> Result<Self, String> {
        Self::new(0, Default::default(), tape)
    }
    pub fn std(tape: Tape<S>) -> Result<Self, String> {
        Self::new(tape.len() - 1, Default::default(), tape)
    }
    pub fn len(&self) -> usize {
        self.tape().len()
    }
    pub fn is_empty(&self) -> bool {
        self.tape().is_empty()
    }
    pub fn position(&self) -> usize {
        self.index
    }
    pub fn set_symbol(&mut self, symbol: S) {
        self.tape.set(self.index, symbol);
    }

    /// Shifts the [`Tape`] to left or right if [`Move`] is [`Move::Left`]
    /// or [`Move::Right`], otherwise do nothing (when [`Move::None`]).
    /// If [`Configuration`] reachs the begin or the end of the [`Tape`]
    /// then [`Tape`] extends by [`Tape::insert`] method, otherwise only
    /// changes self index.
    pub fn shift(&mut self, movement: Move, default: S) {
        match movement as i64 {
            // Left
            0 if self.index == 0 => self.tape.insert(0, default),
            0 => self.index -= 1,
            // Right
            1 => {
                self.index += 1;
                if self.index == self.tape.len() {
                    self.tape.insert(self.index, default);
                }
            }
            // Stay
            _ => {}
        };
    }
    pub fn state(&self) -> State {
        self.state.clone()
    }
    pub fn symbol(&self) -> Option<&S> {
        self.tape.get(self.index)
    }
    pub fn tape(&self) -> Tape<S> {
        self.tape.clone()
    }
}

impl<S: Symbolic> From<Configuration<S>> for (usize, State, Tape<S>) {
    fn from(d: Configuration<S>) -> Self {
        (d.index, d.state, d.tape)
    }
}

impl<S: Symbolic> TryFrom<(usize, State, Tape<S>)> for Configuration<S> {
    type Error = String;

    fn try_from(d: (usize, State, Tape<S>)) -> Result<Self, Self::Error> {
        if d.0 > d.2.len() {
            return Err(format!("The starting position ({}) is out of bounds", d.0));
        }
        Ok(Self {
            index: d.0,
            state: d.1,
            tape: d.2,
        })
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
