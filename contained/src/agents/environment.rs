/*
    Appellation: environment <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Implements a virtual wasm environment; each environment describes a set of capabilities and is responsible for tracing the various results
*/
use crate::prelude::{Shared, State};
use std::sync::{Arc, Mutex};
use wasmer::FunctionEnv;
use wasmer::{imports, Imports, Store};

pub trait Venv {}

pub trait WasmVenv: Send + Sync {
    fn imports(&self, store: &mut Store, with: Option<Imports>) -> Imports;
}

pub struct VirtualEnv {
    pub state: State,
    pub venvs: Vec<Shared<dyn WasmVenv>>,
}
