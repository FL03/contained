/*
    Appellation: environment <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Implements a virtual wasm environment; each environment describes a set of capabilities and is responsible for tracing the various results
*/
//! # Environments
//! 
//! Environments are the primary means of interacting with the WASM runtime. Each environment describes a set of capabilities and is responsible for tracing the various results.
use crate::music::prelude::triads::Triadic;
use wasmer::{imports, FunctionEnv, Imports, Store};

pub trait ThreadSafe: Send + Sync {}

impl<T> ThreadSafe for T where T: Send + Sync {}

pub trait Venv {
    type Env: Clone + WasmEnv;

    /// Returns a reference to the store
    fn store(&self) -> &Store;
    /// Returns a mutable reference to the store
    fn store_mut(&mut self) -> &mut Store;
    /// Returns a  to the environment
    fn venv(&self) -> Self::Env;
    /// Returns a mutable reference to the environment
    fn venv_mut(&mut self) -> &mut Self::Env;
}

pub trait FunctionalVenv: Venv {
    fn function_env(&mut self) -> FunctionEnv<Self::Env>;
}

pub trait WasmEnv: Send + Sync {
    fn imports(&self, store: &mut Store, with: Option<Imports>) -> Imports;
}



impl<T> WasmEnv for T where T: Triadic<Store=Store> {
    fn imports(&self, _store: &mut Store, with: Option<Imports>) -> Imports {
        let mut imports = imports! {
            "env" => {
            }
        };
        if let Some(w) = with {
            imports.extend(&w);
        }
        imports
    }
}

pub struct VirtualEnv {
    env: Box<dyn WasmEnv>,
    store: Store,
}

impl VirtualEnv {
    pub fn new(env: Box<dyn WasmEnv>) -> Self {
        Self {
            env,
            store: Store::default(),
        }
    }

    pub fn imports(&mut self, with: Option<Imports>) -> Imports {
        self.env.imports(&mut self.store, with)
    }

    pub fn store(&self) -> &Store {
        &self.store
    }

    pub fn store_mut(&mut self) -> &mut Store {
        &mut self.store
    }
}
