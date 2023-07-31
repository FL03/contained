/*
    Appellation: agent <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Agent
//! 
//! An agent is an intelligent entity that acts autonomously, directed by its own internal state. 
//! An agent is typically a computer system that is situated in some environment, and that is capable of autonomous action in this environment in order to meet its design objectives.
//! Here, agents are described by their topological execution environments and are capable of executing arbitrary WebAssembly modules.
use super::Context;
use super::layer::Command;

use tokio::{runtime as rt, sync::mpsc, task};
use tracing::instrument;
use wasmer::{Instance, Module, Store};

pub struct AgentBuilder {
    params: Option<AgentParams>,
}

pub struct AgentParams {
    pub name: String,
}

pub struct Agent {
    cmd: mpsc::Receiver<Command>,
    context: Context,
    store: Store
}

impl Agent {
    pub fn new(cmd: mpsc::Receiver<Command>, context: Context, store: Store) -> Self {
        Self { cmd, context, store }
    }
    pub fn with_capacity(capacity: usize, context: Context, store: Store) -> (Self, mpsc::Sender<Command>) {
        let (tx, cmd) = mpsc::channel(capacity);
        (Self::new(cmd, context, store), tx)
    }

    pub fn context(&self) -> Context {
        self.context.clone()
    }


    #[instrument(err, skip(self, cmd), name = "process", target = "agent")]
    pub async fn process(&mut self, cmd: Command) -> anyhow::Result<()> {
        match cmd {
            Command::Execute {
                module,
                function,
                args,
                with,
                tx,
            } => {
                let stack = &self.context.stack();
                let modules = stack.modules().read().unwrap();
                tracing::debug!("Fetching the program...");
                let module = modules.get(&module).unwrap();
                tracing::debug!("Importing host functions");
                let imports = self
                    .context
                    .env()
                    .lock()
                    .unwrap()
                    .imports(&mut self.store_mut(), with);
                tracing::info!("Instantiating module with the imported host functions");
                let instance = Instance::new(&mut self.store_mut(), &module, &imports)
                    .expect("Failed to instantiate module");
                tracing::info!("Fetching the function");
                let func = instance.exports.get_function(&function)?;
                tracing::info!("Executing the function with the provided arguments");
                let result = func.call(&mut self.store_mut(), &args)?;
                tx.send(Ok(result)).unwrap();
                Ok(())
            }
            Command::Include { bytes, tx } => {
                let module = Module::new(self.store(), bytes)?;
                let hash = self.context
                    .stack()
                    .add_module(module);
                tx.send(Ok(hash.into())).unwrap();
                Ok(())
            }
            Command::Transform { .. } => todo!(),
        }
    }
    #[instrument(skip(self), name = "run", target = "agent")]
    pub async fn run(mut self) {
        loop {
            tokio::select! {
                Some(cmd) = self.cmd.recv() => {
                    tracing::debug!("Processing command");
                    self.process(cmd).await.expect("Failed to process command");
                }
                _ = tokio::signal::ctrl_c() => {
                    tracing::warn!("Signal received, shutting down");
                    break;
                }
            }
        }
    }
    pub fn spawn(self, handle: rt::Handle) -> task::JoinHandle<()> {
        handle.spawn(self.run())
    }

    pub fn store(&self) -> &Store {
        &self.store
    }

    pub fn store_mut(&mut self) -> &mut Store {
        &mut self.store
    }


}
