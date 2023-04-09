/*
    Appellation: agent <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: An agent describes a persistent, stateful, and isolated virtual machine.
*/
use super::{client::AgentManager, layer::Command, Stack, VirtualEnv};
use crate::prelude::{hash_module, Shared, State};
use scsys::prelude::AsyncResult;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use wasmer::{Instance, Module, Store};

pub struct Agent {
    cmd: mpsc::Receiver<Command>,
    env: Shared<VirtualEnv>,
    stack: Shared<Stack>,
    state: Shared<State>,
    store: Store,
}

impl Agent {
    pub fn new(buffer: usize) -> (Self, impl AgentManager) {
        let (tx, cmd) = mpsc::channel(buffer);
        (
            Self {
                cmd,
                env: Arc::new(Mutex::new(VirtualEnv::default())),
                stack: Arc::new(Mutex::new(Stack::new())),
                state: Arc::new(Mutex::new(State::default())),
                store: Store::default(),
            },
            tx,
        )
    }
    pub async fn process(&mut self, cmd: Command) -> AsyncResult {
        match cmd {
            Command::Execute {
                module,
                function,
                args,
                with,
                tx,
            } => {
                let stack = &self.stack.lock().unwrap();
                let modules = stack.modules.read().unwrap();
                tracing::debug!("Fetching the program...");
                let module = modules.get(&module).unwrap();
                tracing::debug!("Importing host functions");
                let imports = self.env.lock().unwrap().imports(&mut self.store, with);
                tracing::info!("Instantiating module with the imported host functions");
                let instance = Instance::new(&mut self.store, &module, &imports)
                    .expect("Failed to instantiate module");
                tracing::info!("Fetching the function");
                let func = instance.exports.get_function(&function)?;
                tracing::info!("Executing the function with the provided arguments");
                let result = func.call(&mut self.store, &args)?;
                tx.send(Ok(result)).unwrap();
                Ok(())
            }
            Command::Include { bytes, tx } => {
                let module = Module::new(&self.store, bytes)?;
                let hash = hash_module(module.clone());
                self.stack
                    .lock()
                    .unwrap()
                    .modules
                    .write()
                    .unwrap()
                    .insert(hash.into(), module);
                tx.send(Ok(hash.into())).unwrap();
                Ok(())
            }
            Command::Transform { .. } => todo!(),
        }
    }
    pub fn set_environment(mut self, env: VirtualEnv) -> Self {
        self.env = Arc::new(Mutex::new(env));
        self
    }
    pub async fn run(mut self) -> AsyncResult {
        Ok(loop {
            tokio::select! {
                Some(cmd) = self.cmd.recv() => {
                    tracing::debug!("Processing command");
                    self.process(cmd).await?;
                }
                _ = tokio::signal::ctrl_c() => {
                    tracing::warn!("Signal received, shutting down");
                    break;
                }
            }
        })
    }
    pub fn spawn(self, handle: tokio::runtime::Handle) -> tokio::task::JoinHandle<AsyncResult> {
        handle.spawn(self.run())
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
