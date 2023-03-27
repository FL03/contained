/*
    Appellation: space <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A space describes the environment in-which wasm modules may be executed in

*/
use crate::core::{State, Stateful};
use crate::music::{neo::triads::*, Note};

use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Space {
    state: State,
    triad: Triad,
}

impl Space {
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

impl Stateful<State> for Space {
    fn state(&self) -> State {
        self.state
    }

    fn update_state(&mut self, state: State) {
        self.state = state;
    }
}


