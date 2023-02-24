/*
    Appellation: config <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::turing::{Move, Symbolic, Tape};
use crate::{State, States};

use scsys::prelude::StatePack;
use serde::{Deserialize, Serialize};

pub trait Configurable<S: Symbolic>: Clone {
    type State: Clone + StatePack;
    /// [Configurable::is_empty] is a method for checking if the tape is empty
    fn is_empty(&self) -> bool {
        self.tape().is_empty()
    }
    /// [Configurable::len] describes a method which returns the number of elements currently in the [Tape]
    fn len(&self) -> usize {
        self.tape().len()
    }
    /// [Configurable::set_index] is a method for modifying the scope or positioning of the machine
    fn set_index(&mut self, pos: usize);
    /// [Configurable::set_state] is a method for changing the current state of the machine
    fn set_state(&mut self, state: State<Self::State, S>);
    /// [Configurable::set_symbol] is a method for changing the symbol at the current position
    fn set_symbol(&mut self, symbol: S);
    /// [Configurable::shift] Shifts the [`Tape`] to left or right if [`Move`] is [`Move::Left`]
    /// or [`Move::Right`], otherwise do nothing (when [`Move::None`]).
    /// If [`Configuration`] reachs the begin or the end of the [`Tape`]
    /// then [`Tape`] extends by [`Tape::insert`] method, otherwise only
    /// changes self index.
    fn shift(&mut self, movement: Move, default: S) {
        match movement as i64 {
            // Left
            0 if self.position() == 0 => self.mut_tape().insert(0, default),
            0 => self.set_index(self.position() - 1),
            // Right
            1 => {
                self.set_index(self.position() + 1);
                if self.position() == self.mut_tape().len() {
                    self.set_symbol(default);
                }
            }
            // Stay
            _ => {}
        };
    }
    /// [Configurable::position] returns the current position of the machine
    fn position(&self) -> usize;
    /// [Configurable::state] returns an owned instance of the machine [State]
    fn state(&self) -> &State<Self::State, S>;
    /// [Configurable::symbol] returns an owned instance of the symbol at the current position of the machine
    fn symbol(&self) -> &S {
        self.tape()
            .get(self.position())
            .expect("The index is currently out of bounds...")
    }
    /// [Configurable::tape] returns an owned instance of the machines [Tape]
    fn tape(&self) -> &Tape<S>;
    /// [Configurable::mut_tape] returns an owned, mutable instance of the machines [Tape]
    fn mut_tape(&mut self) -> &mut Tape<S>;
}

pub enum Configurations<S: Symbolic> {
    Normal(Configuration<S>),
    Standard(Configuration<S>),
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Configuration<S: Symbolic> {
    index: usize,
    pub(crate) state: State<States, S>,
    tape: Tape<S>,
}

impl<S: Symbolic> Configuration<S> {
    pub fn build(index: usize, state: State<States, S>, tape: Tape<S>) -> Result<Self, String> {
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
}

impl<S: Symbolic> Configurable<S> for Configuration<S> {
    type State = States;

    fn set_index(&mut self, pos: usize) {
        self.index = pos;
    }

    fn set_state(&mut self, state: State<Self::State, S>) {
        self.state = state;
    }

    fn set_symbol(&mut self, symbol: S) {
        self.tape.set(self.position(), symbol);
    }
    fn position(&self) -> usize {
        self.index
    }

    fn state(&self) -> &State<Self::State, S> {
        &self.state
    }

    fn tape(&self) -> &Tape<S> {
        &self.tape
    }

    fn mut_tape(&mut self) -> &mut Tape<S> {
        &mut self.tape
    }
}

impl<S: Symbolic> From<Configuration<S>> for (usize, State<States, S>, Tape<S>) {
    fn from(d: Configuration<S>) -> Self {
        (d.index, d.state, d.tape)
    }
}

impl<S: Symbolic> TryFrom<(usize, State<States, S>, Tape<S>)> for Configuration<S> {
    type Error = String;

    fn try_from(d: (usize, State<States, S>, Tape<S>)) -> Result<Self, Self::Error> {
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
