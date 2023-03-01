/*
    Appellation: config <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::turing::{Move, Tape};
use crate::{State, States, Symbolic};

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
)]
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum Config {
    #[default]
    Normal = 0,
    Standard = 1,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Configuration<S: Symbolic = String> {
    index: usize,
    pub(crate) state: State<States>,
    tape: Tape<S>,
}

impl<S: Symbolic> Configuration<S> {
    pub fn new(index: usize, state: State<States>, tape: Tape<S>) -> Self {
        Self { index, state, tape }
    }
    pub fn create(tape: Tape<S>, config: Option<Config>) -> Result<Self, String> {
        let cnf = match config.unwrap_or_default() {
            Config::Normal => (0, Default::default(), tape),
            Config::Standard => (tape.len() - 1, Default::default(), tape),
        };
        Self::try_from(cnf)
    }
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
        Self::create(tape, Some(Config::Normal))
    }
    pub fn std(tape: Tape<S>) -> Result<Self, String> {
        Self::create(tape, Some(Config::Standard))
    }
    /// [Configuration::is_empty]
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
    pub fn state(&self) -> &State<States> {
        &self.state
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

impl<S: Symbolic> TryFrom<(usize, State<States>, Tape<S>)> for Configuration<S> {
    type Error = String;
    fn try_from(d: (usize, State<States>, Tape<S>)) -> Result<Self, Self::Error> {
        if d.0 > d.2.len() {
            return Err("Starting index is out of bounds...".to_string());
        }
        Ok(Self::new(d.0, d.1, d.2))
    }
}

impl<S: Symbolic> From<Configuration<S>> for (usize, State<States>, Tape<S>) {
    fn from(d: Configuration<S>) -> Self {
        (d.index, d.state, d.tape)
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
