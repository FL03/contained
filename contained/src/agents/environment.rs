/*
    Appellation: environment <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Implements a virtual wasm environment; each environment describes a set of capabilities and is responsible for tracing the various results
*/
use crate::music::neo::triads::Triad;
use crate::prelude::Shared;
use wasmer::{imports, Imports, Store};

pub trait WasmVenv: Send + Sync {
    fn imports(&self, store: &mut Store, with: Option<Imports>) -> Imports;
}

impl WasmVenv for () {
    fn imports(&self, _store: &mut Store, _with: Option<Imports>) -> Imports {
        imports! {}
    }
}

impl WasmVenv for Triad {
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
    pub scope: Shared<Triad>,
    pub env: Box<dyn WasmVenv>,
}
