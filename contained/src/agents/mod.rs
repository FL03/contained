/*
    Appellation: agents <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: An agent describes a persistent, stateful, and isolated virtual machine.


*/
use crate::prelude::{Shared, State};
use crate::vm::VirtualEnv;
use std::sync::{Arc, Mutex};
use wasmer::{imports, Imports, Module, Store};
use wasmer::{Function, FunctionEnv, FunctionEnvMut};


pub trait WasmAgent {
    type Env;
    fn env(&self) -> &Shared<Self::Env>;
    fn import(&mut self) -> Imports;
    fn store(&self) -> &Store;
    fn store_mut(&mut self) -> &mut Store;
    fn state(&self) -> &Shared<State>;
}

pub struct Agent {
    pub env: Shared<VirtualEnv>,
    pub state: Shared<State>,
    pub store: Store,
}

impl Agent {
    pub fn new() -> Self {
        let env = Arc::new(Mutex::new(VirtualEnv::default()));
        let state = Arc::new(Mutex::new(State::default()));
        let store = Store::default();
        Self { env, state, store }
    }

    pub fn import(&mut self) -> Imports {
        let env = self.env.lock().unwrap().function_env(&mut self.store);
        imports! {
            "env" => {}
        }
    }
}
