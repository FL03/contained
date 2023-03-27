/*
    Appellation: space <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A space describes the environment in-which wasm modules may be executed in;

*/
use crate::core::{AsyncStateful, Shared, State};
use crate::music::neo::triads::*;

use std::sync::{Arc, Mutex};
use wasmer::{FunctionEnv, AsStoreMut};

#[derive(Clone, Debug, Default)]
pub struct Space {
    space: Shared<Triad>,
    state: Shared<State>,
}

impl Space {
    pub fn new(space: Triad) -> Self {
        Self {
            space: Arc::new(Mutex::new(space)),
            state: Arc::new(Mutex::new(State::Valid)),
        }
    }
    pub fn function_env(&self, store: &mut impl AsStoreMut) -> FunctionEnv<Self> {
        FunctionEnv::new(store, self.clone())
    }
    pub fn state(&self) -> Shared<State> {
        self.state.clone()
    }
    pub fn space(&self) -> Shared<Triad> {
        self.space.clone()
    }
}

impl AsyncStateful<State> for Space {
    fn state(&self) -> Shared<State> {
        self.state.clone()
    }
    fn update_state(&mut self, state: Shared<State>) {
        self.state = state;
    }
}