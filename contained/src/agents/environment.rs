/*
    Appellation: environment <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Implements a virtual wasm environment; each environment describes a set of capabilities and is responsible for tracing the various results
*/
use crate::music::neo::triads::Triad;
use wasmer::{imports, FunctionEnv, Imports, Store};

pub trait ThreadSafe: Send + Sync {}

impl<T> ThreadSafe for T where T: Send + Sync {}

pub trait Venv<Env>
where
    Env: Send + Sync,
{
    fn function_env(&mut self) -> FunctionEnv<Env>;
    /// Returns the imports for the environment; optionally with additional imports
    fn imports(&self, with: Option<Imports>) -> Imports;
    /// Returns a reference to the store
    fn store(&self) -> &Store;
    /// Returns a mutable reference to the store
    fn store_mut(&mut self) -> &mut Store;
    /// Returns a  to the environment
    fn venv(&self) -> &Env;
    /// Returns a mutable reference to the environment
    fn venv_mut(&mut self) -> &mut Env;
}

pub trait WasmEnv: Send + Sync {
    fn imports(&self, store: &mut Store, with: Option<Imports>) -> Imports;
}

impl WasmEnv for () {
    fn imports(&self, _store: &mut Store, _with: Option<Imports>) -> Imports {
        imports! {}
    }
}

impl WasmEnv for Triad {
    fn imports(&self, _store: &mut Store, with: Option<Imports>) -> Imports {
        let mut imports = imports! {
            "env" => {}
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
