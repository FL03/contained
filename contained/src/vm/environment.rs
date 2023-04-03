/*
    Appellation: environment <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Implements a virtual wasm environment; each environment describes a set of capabilities and is responsible for tracing the various results
*/
use crate::prelude::{Shared, State};
use std::sync::{Arc, Mutex};
use wasmer::{imports, Imports, Store};
use wasmer::{Function, FunctionEnv, FunctionEnvMut};



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
    pub fn imports(&self, store: &mut Store) -> Imports {
        let get_counter =
            |env: FunctionEnvMut<VirtualEnv>| -> i32 { *env.data().value.lock().unwrap() };
        let add_to_counter = |env: FunctionEnvMut<VirtualEnv>, add: i32| -> i32 {
            let mut counter_ref = env.data().value.lock().unwrap();

            *counter_ref += add;
            *counter_ref
        };
        let env = self.function_env(store);
        let get_counter_func = Function::new_typed_with_env(store, &env, get_counter);

        let add_to_counter_func = Function::new_typed_with_env(store, &env, add_to_counter);

        imports! {
            "env" => {
                "get_counter" => get_counter_func,
                "add_to_counter" => add_to_counter_func,
            }
        }
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
