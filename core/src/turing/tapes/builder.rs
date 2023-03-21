/*
    Appellation: builder <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::Tape;
use crate::Symbolic;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumVariantNames};

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]

pub enum Tapes<S: Symbolic = String> {
    Normal(Tape<S>),
    Standard(Tape<S>),
}

impl<S: Symbolic> Tapes<S> {
    pub fn norm(iter: impl IntoIterator<Item = S>) -> Self {
        Self::Normal(Tape::new(iter))
    }
    pub fn std(iter: impl IntoIterator<Item = S>) -> Self {
        Self::Standard(Tape::new(iter))
    }
    pub fn build(&self) -> Tape<S> {
        match self.clone() {
            Self::Normal(tape) => tape,
            Self::Standard(tape) => tape,
        }
    }
}

impl<S: Symbolic> Default for Tapes<S> {
    fn default() -> Self {
        Self::Normal(Default::default())
    }
}

impl<S: Symbolic> From<Tapes<S>> for Tape<S> {
    fn from(tape: Tapes<S>) -> Tape<S> {
        match tape {
            Tapes::Normal(t) => t,
            Tapes::Standard(t) => t,
        }
    }
}
