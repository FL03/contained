/*
    Appellation: config <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::turing::{Move, Tape};
use crate::{State, Stateful, States, Symbolic};

use serde::{Deserialize, Serialize};

pub enum Configurations<S: Symbolic> {
    Normal(Configuration<S>),
    Standard(Configuration<S>),
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Configuration<S: Symbolic = String> {
    index: usize,
    pub(crate) state: State<States>,
    tape: Tape<S>,
}

impl<S: Symbolic> Configuration<S> {
    pub fn build(index: usize, state: State<States>, tape: Tape<S>) -> Result<Self, String> {
        if index > tape.len() {
            return Err(format!(
                "The starting position ({}) is out of bounds",
                index
            ));
        }
        Ok(Self { index, state, tape })
    }
    pub fn norm(tape: Tape<S>) -> Result<Self, String> {
        Self::build(0, Default::default(), tape)
    }
    pub fn std(tape: Tape<S>) -> Result<Self, String> {
        Self::build(tape.len() - 1, Default::default(), tape)
    }
    /// [Configurable::is_empty] is a method for checking if the tape is empty
    pub fn is_empty(&self) -> bool {
        self.tape().is_empty()
    }
    /// [Configurable::len] describes a method which returns the number of elements currently in the [Tape]
    pub fn len(&self) -> usize {
        self.tape().len()
    }

    pub fn set_symbol(&mut self, symbol: S) {
        self.tape.set(self.position(), symbol);
    }

    /// [Configurable::shift] Shifts the [`Tape`] to left or right if [`Move`] is [`Move::Left`]
    /// or [`Move::Right`], otherwise do nothing (when [`Move::None`]).
    /// If [`Configuration`] reachs the begin or the end of the [`Tape`]
    /// then [`Tape`] extends by [`Tape::insert`] method, otherwise only
    /// changes self index.
    pub fn shift(&mut self, movement: Move, default: S) {
        match movement as i64 {
            // Left
            0 if self.position() == 0 => self.tape.insert(0, default),
            0 => self.index -= 1,
            // Right
            1 => {
                self.index += 1;
                if self.position() == self.tape.len() {
                    self.set_symbol(default);
                }
            }
            // Stay
            _ => {}
        };
    }
    pub fn position(&self) -> usize {
        self.index
    }
    pub fn symbol(&self) -> &S {
        self.tape
            .get(self.position())
            .expect("index is out of bounds...")
    }
    pub fn tape(&self) -> &Tape<S> {
        &self.tape
    }
}

impl<S: Ord + Symbolic> std::fmt::Display for Configuration<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {:?}",
            self.index,
            self.state().to_string(),
            self.tape()
        )
    }
}

impl<S: Ord + Symbolic> Stateful<States> for Configuration<S> {
    fn state(&self) -> &States {
        self.state.state()
    }
}

impl<S: Symbolic> From<Configuration<S>> for (usize, State<States>, Tape<S>) {
    fn from(d: Configuration<S>) -> Self {
        (d.index, d.state, d.tape)
    }
}

impl<S: Symbolic> TryFrom<(usize, State<States>, Tape<S>)> for Configuration<S> {
    type Error = String;

    fn try_from(d: (usize, State<States>, Tape<S>)) -> Result<Self, Self::Error> {
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
