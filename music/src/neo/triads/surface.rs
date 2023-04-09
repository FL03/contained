/*
    Appellation: surface <triads>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Generically, a surface describes a type of topological compute surface. Here we implement a surface for triads, which are the fundamental unit of computation in contained.
*/
use super::*;
use crate::neo::{Transform, LPR};
use contained_core::{AsyncStateful, Shared, State};
use decanter::prelude::Hashable;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Default, Hashable)]
pub struct Surface {
    state: Shared<State>,
    triad: Shared<Triad>,
}

impl Surface {
    pub fn new(triad: Triad) -> Self {
        Self {
            state: Arc::new(Mutex::new(State::default())),
            triad: Arc::new(Mutex::new(triad)),
        }
    }
    pub fn transform(&mut self, lpr: LPR) {
        self.state.lock().unwrap().invalidate();
        self.triad.lock().unwrap().transform(lpr);
        self.state.lock().unwrap().validate();
    }
    pub fn triad(&self) -> Shared<Triad> {
        self.triad.clone()
    }
}

impl AsyncStateful<State> for Surface {
    fn state(&self) -> Shared<State> {
        self.state.clone()
    }

    fn update_state(&mut self, state: Shared<State>) {
        self.state = state;
    }
}

impl std::fmt::Display for Surface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = serde_json::json!(
            {
                "state": self.state.lock().unwrap().clone().to_string(),
                "triad": self.triad.lock().unwrap().clone().to_string(),
            }
        );
        write!(f, "{}", msg)
    }
}
