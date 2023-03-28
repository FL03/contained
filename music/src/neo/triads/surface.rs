/*
    Appellation: surface <triads>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A surface is used to describe the actual environment in-which the WebAssembly engines are running in. 
        Each surface is a stateful instance of a triad 
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
