/*
    Appellation: instance <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: An instance is a stateful, self-contained, and self-executing machine that is used to describe a computable surface.
        Generally, for a surface to be computable it must contain a set of vertices connected by edges where the surface contained within or area of the shape is stateful.

        The surface of the triad is described by its states while the triad itself is used to describe the subset of notes that are contained within the surface.
        That being said it is important to consider that each surface is better understood as the environment in-which a WASM machine is operating.
*/
use super::Triad;
use crate::Note;
use contained_core::actors::Actor;
use contained_core::turing::{Driver, Machine, Program, Tape};
use contained_core::{State, Stateful};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Instance {
    state: State,
    triad: Triad,
}

impl Instance {
    pub fn new(triad: Triad) -> Self {
        Self {
            state: State::default(),
            triad,
        }
    }
    /// Create a new [Actor] with the [Triad] as its alphabet
    pub fn actor(&self, tape: Option<Tape<Note>>) -> Actor<Note> {
        Actor::new(self.program(), tape)
    }
    /// Initializes a new instance of a [Machine] configured with the current alphabet
    pub fn machine(&self, tape: Option<Tape<Note>>) -> Machine<Note> {
        Machine::new(
            Driver::new(State::Valid, tape.unwrap_or_default()),
            self.program(),
        )
    }

    pub fn program(&self) -> Program<Note> {
        Program::new(self.triad(), State::Invalid)
    }
    pub fn triad(&self) -> Triad {
        self.triad.clone()
    }
}

impl Stateful<State> for Instance {
    fn state(&self) -> State {
        self.state
    }

    fn update_state(&mut self, state: State) {
        self.state = state;
    }
}
