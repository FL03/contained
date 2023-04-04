/*
    Appellation: environment <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Implements a virtual wasm environment; each environment describes a set of capabilities and is responsible for tracing the various results
*/
use crate::prelude::{Shared, State};
use std::sync::{Arc, Mutex};
use wasmer::{imports, Imports, Store};
use wasmer::FunctionEnv;

#[derive(Clone, Debug)]
pub struct VirtualEnv {
    pub state: Shared<State>,
    pub value: Shared<i32>,
}

impl VirtualEnv {
    pub fn new(value: i32) -> Self {
        Self {
            state: Arc::new(Mutex::new(State::default())),
            value: Arc::new(Mutex::new(value)),
        }
    }
    pub fn function_env(&self, store: &mut Store) -> FunctionEnv<Self> {
        FunctionEnv::new(store, self.clone())
    }
    pub fn imports(&self, store: &mut Store, with: Option<Imports>) -> Imports {
        let mut base = imports! {
            "env" => {}
        };
        if let Some(with) = with {
            base.extend(&with);
        }
        base
    }
}

impl Default for VirtualEnv {
    fn default() -> Self {
        Self::new(0)
    }
}

impl From<Shared<i32>> for VirtualEnv {
    fn from(value: Shared<i32>) -> Self {
        Self {
            state: Arc::new(Mutex::new(State::default())),
            value,
        }
    }
}
