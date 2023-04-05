/*
    Appellation: agent <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: An agent describes a persistent, stateful, and isolated virtual machine.
*/
use super::{layer::Command, Stack, VirtualEnv};
use crate::prelude::{hash_module, Shared, State};
use scsys::prelude::AsyncResult;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use wasmer::{Instance, Module, Store};

pub struct Agent {
    pub cmd: mpsc::Receiver<Command>,
    pub env: Shared<VirtualEnv>,
    pub stack: Shared<Stack>,
    pub state: Shared<State>,
    pub store: Store,
}

impl Agent {
    pub fn new(buffer: usize) -> (Self, mpsc::Sender<Command>) {
        let (cmd, rx) = mpsc::channel(buffer);
        (
            Self {
                cmd: rx,
                env: Arc::new(Mutex::new(VirtualEnv::default())),
                stack: Arc::new(Mutex::new(Stack::new())),
                state: Arc::new(Mutex::new(State::default())),
                store: Store::default(),
            },
            cmd,
        )
    }
    pub async fn handle_command(&mut self, cmd: Command) -> AsyncResult {
        match cmd {
            Command::Include { bytes, sender } => {
                let module = Module::new(&self.store, bytes)?;
                let hash = hash_module(module.clone());
                self.stack
                    .lock()
                    .unwrap()
                    .modules
                    .insert(hash.into(), module);
                sender.send(Ok(hash.into())).unwrap();
                Ok(())
            }
            Command::Execute {
                module,
                function,
                args,
                with,
                sender,
            } => {
                let modules = self.stack.lock().unwrap().modules.clone();
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
                sender.send(Ok(result)).unwrap();
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
                    self.handle_command(cmd).await?;
                }
                _ = tokio::signal::ctrl_c() => {
                    tracing::warn!("Signal received, shutting down");
                    break;
                }
            }
        })
    }
    pub fn spawn(self) -> tokio::task::JoinHandle<AsyncResult> {
        tokio::spawn(self.run())
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
