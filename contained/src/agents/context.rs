/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Stack, WasmEnv};
use std::sync::{Arc, Mutex};
use tracing::instrument;
use wasmer::Store;

pub struct Context {
    env: Arc<Mutex<Box<dyn WasmEnv>>>,
    stack: Arc<Mutex<Stack>>,
    store: Store,
}

impl Context {
    pub fn new(env: Box<dyn WasmEnv>, stack: Stack, store: Store) -> Self {
        Self {
            env: Arc::new(Mutex::new(env)),
            stack: Arc::new(Mutex::new(stack)),
            store,
        }
    }

    pub fn env(&self) -> Arc<Mutex<Box<dyn WasmEnv>>> {
        self.env.clone()
    }
    #[instrument(skip(self, env), name = "environment", target = "context")]
    pub fn set_environment(mut self, env: Box<dyn WasmEnv>) -> Self {
        self.env = Arc::new(Mutex::new(env));
        self
    }

    pub fn stack(&self) -> Stack {
        self.stack.lock().unwrap().clone()
    }

    pub fn store(&self) -> &Store {
        &self.store
    }

    pub fn store_mut(&mut self) -> &mut Store {
        &mut self.store
    }

    pub fn with_stack(mut self, stack: Stack) -> Self {
        self.stack = Arc::new(Mutex::new(stack));
        self
    }
    pub fn with_store(mut self, store: Store) -> Self {
        self.store = store;
        self
    }
}
