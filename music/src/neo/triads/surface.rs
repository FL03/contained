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
use contained_core::turing::Program;
use contained_core::{State, Stateful};

use serde::{Deserialize, Serialize};

pub trait Surfaced<T> {
    fn edges(&self) -> Vec<(T, T)>;
    fn vertices(&self) -> Vec<T>;
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Surface {
    state: State,
    triad: Triad,
}

impl Surface {
    pub fn new(triad: Triad) -> Self {
        Self {
            state: State::default(),
            triad,
        }
    }
    pub fn triad(&self) -> Triad {
        self.triad.clone()
    }
}

impl Stateful<State> for Surface {
    fn state(&self) -> State {
        self.state
    }

    fn update_state(&mut self, state: State) {
        self.state = state;
    }
}

impl From<Surface> for Program<Note> {
    fn from(surface: Surface) -> Program<Note> {
        Program::new(surface.triad(), State::Invalid)
    }
}
