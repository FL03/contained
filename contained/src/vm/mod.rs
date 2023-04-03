/*
    Appellation: vm <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{client::*, environment::*};

mod client;
mod environment;

use crate::music::neo::LPR;
use crate::prelude::{BoxedWasmValue, Shared};
use scsys::prelude::AsyncResult;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use wasmer::{Instance, Module, Store, TypedFunction};

pub enum Operations {
    Connect { host: String, port: u16 },
    Disconnect,
    Execute { module: Module },
    Queue { module: Module },
    Transform { dirac: LPR },
}

pub enum VMCommand {
    Add { module: Module },
    Remove { module: Module },
}

pub enum VMFrame {
    Request {},
    Response {},
}

#[derive(Debug)]
pub struct Computer {
    env: Shared<VirtualEnv>,
    program: mpsc::Receiver<Module>,
    results: mpsc::Sender<BoxedWasmValue>,
    store: Store,
    transform: mpsc::Receiver<String>,
}

impl Computer {
    pub fn new(
        program: mpsc::Receiver<Module>,
        results: mpsc::Sender<BoxedWasmValue>,
        transform: mpsc::Receiver<String>,
    ) -> Self {
        Self {
            env: Arc::new(Mutex::new(VirtualEnv::default())),
            program,
            results,
            store: Store::default(),
            transform,
        }
    }
    pub async fn run(mut self) -> AsyncResult {
        Ok(loop {
            tokio::select! {
                Some(module) = self.program.recv() => {
                    tracing::info!("Received a new program");
                    let host = self.env.lock().unwrap().imports(&mut self.store);
                    tracing::info!("Instantiating module with the imported host functions");
                    let instance = Instance::new(&mut self.store, &module, &host).expect("Failed to instantiate module");
                    tracing::info!("Success: Instantiated module with the imported host functions");
                    let run: TypedFunction<i32, i32> = instance.exports.get_function("increment")?.typed(&mut self.store)?;
                    tracing::info!("Success: Got the counter function from the module");
                    let result = run.call(&mut self.store, 5)?;
                    tracing::info!("Success: Ran the counter function from the module\n\tCounter value (host): {:?}", result);
                    self.results.send(Box::new([result.into()])).await?;
                }
                Some(transform) = self.transform.recv() => {
                    println!("{:?}", transform);
                }
                _ = tokio::signal::ctrl_c() => {
                    tracing::warn!("Signal received, shutting down");
                    break;
                }
                else => tracing::warn!("Tonic has no more work to do"),
            }
        })
    }
    pub fn set_environment(mut self, env: VirtualEnv) -> Self {
        self.env = Arc::new(Mutex::new(env));
        self
    }
    pub fn spawn(self) -> tokio::task::JoinHandle<AsyncResult> {
        tokio::spawn(self.run())
    }
}
