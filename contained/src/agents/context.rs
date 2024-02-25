/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Stack, WasmEnv};
use std::sync::{Arc, Mutex};
use tracing::instrument;
use wasmer::{AsEngineRef, Engine, Store};

#[derive(Clone)]
pub struct Context {
    engine: Engine,
    env: Arc<Mutex<Box<dyn WasmEnv>>>,

    stack: Stack,
}

impl Context {
    pub fn new(engine: impl AsEngineRef, env: Box<dyn WasmEnv>, stack: Stack) -> Self {
        Self {
            engine: engine.as_engine_ref().engine().clone(),
            env: Arc::new(Mutex::new(env)),
            stack,
        }
    }

    pub fn engine(&self) -> Engine {
        self.engine.clone()
    }

    pub fn env(&self) -> Arc<Mutex<Box<dyn WasmEnv>>> {
        self.env.clone()
    }

    pub fn stack(&self) -> Stack {
        self.stack.clone()
    }

    pub fn stack_mut(&mut self) -> &mut Stack {
        &mut self.stack
    }

    pub fn store(&self) -> Store {
        Store::new(self.engine())
    }

    pub fn with_engine(mut self, engine: Engine) -> Self {
        self.engine = engine;
        self
    }

    #[instrument(skip(self, env), name = "environment", target = "context")]
    pub fn with_environment(mut self, env: Box<dyn WasmEnv>) -> Self {
        self.env = Arc::new(Mutex::new(env));
        self
    }

    pub fn with_stack(mut self, stack: Stack) -> Self {
        self.stack = stack;
        self
    }
}
