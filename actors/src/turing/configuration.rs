/*
    Appellation: config <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::turing::{Symbolic, Tape};
use crate::{Scope, State, States};

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
    pub fn build(tape: Tape<S>, config: Option<Config>) -> Self {
        let cnf = match config.unwrap_or_default() {
            Config::Normal => (0, Default::default(), tape),
            Config::Standard => (tape.len() - 1, Default::default(), tape),
        };
        // All descirbed
        Self::try_from(cnf).unwrap()
    }
    /// [Configuration::is_empty]
    pub fn is_empty(&self) -> bool {
        self.tape().is_empty()
    }
    /// [Configurable::len] describes a method which returns the number of elements currently in the [Tape]
    pub fn len(&self) -> usize {
        self.tape().len()
    }
}

impl<S: Symbolic> Scope<S> for Configuration<S> {
    fn insert(&mut self, elem: S) {
        self.tape.insert(self.position(), elem);
    }

    fn position(&self) -> usize {
        self.index
    }

    fn set_position(&mut self, index: usize) {
        self.index = index;
    }
    fn set_state(&mut self, state: State<States>) {
        self.state = state;
    }
    fn set_symbol(&mut self, elem: S) {
        self.tape.set(self.position(), elem);
    }

    fn state(&self) -> &State<States> {
        &self.state
    }

    fn tape(&self) -> &Tape<S> {
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
        let a = Configuration::build(tape.clone(), None);
        let b = Configuration::build(tape, Some(Config::Standard));
        assert_ne!(a, b);
    }
}
