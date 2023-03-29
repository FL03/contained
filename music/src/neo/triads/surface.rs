/*
    Appellation: surface <triads>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Generically, a surface describes a type of topological compute surface. Here we implement a surface for triads, which are the fundamental unit of computation in contained.
*/
use super::Triad;
use contained_core::{State, Stateful};
use serde::{Deserialize, Serialize};

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
