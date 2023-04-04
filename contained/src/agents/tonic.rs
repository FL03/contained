/*
    Appellation: tonic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A tonic is traditionally used to to describe the first degree (note) of the diatonic scale.
        Here, a tonic is used to describe a unit-computing machine that is capable of either executing a program or transforming its scope.
*/
use crate::music::neo::{triads::{Surface, Triad}, LPR};
use crate::prelude::{BoxedWasmValue, Shared};
use scsys::prelude::AsyncResult;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use wasmer::{Instance, Module, Store};

#[derive(Debug)]
pub struct TonicChannel {
    pub program: mpsc::Receiver<Module>,
    pub results: mpsc::Sender<BoxedWasmValue>,
    pub transform: mpsc::Receiver<LPR>,
}

impl TonicChannel {
    pub fn new(
        program: mpsc::Receiver<Module>,
        results: mpsc::Sender<BoxedWasmValue>,
        transform: mpsc::Receiver<LPR>,
    ) -> Self {
        Self {
            program,
            results,
            transform,
        }
    }
    pub fn program(&self) -> &mpsc::Receiver<Module> {
        &self.program
    }
    pub fn transform(&self) -> &mpsc::Receiver<LPR> {
        &self.transform
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self::new(
            mpsc::channel(capacity).1,
            mpsc::channel(capacity).0,
            mpsc::channel(capacity).1,
        )
    }
}

impl Default for TonicChannel {
    fn default() -> Self {
        Self::with_capacity(9)
    }
}

#[derive(Debug)]
pub struct Tonic {
    chan: TonicChannel,
    store: Store,
    surface: Shared<Surface>,
}

impl Tonic {
    pub fn new(chan: TonicChannel, scope: Triad) -> Self {
        Self {
            chan,
            store: Store::default(),
            surface: Arc::new(Mutex::new(Surface::new(scope))),
        }
    }
    pub async fn run(mut self) -> AsyncResult {
        Ok(loop {
            tokio::select! {
                Some(module) = self.chan.program.recv() => {
                    tracing::info!("Received a new program");
                    let host = self.surface.lock().unwrap().imports(&mut self.store);
                    tracing::info!("Success: Imported host functions");
                    let instance = Instance::new(&mut self.store, &module, &host).expect("Failed to instantiate module");
                    tracing::info!("Success: Initialized a new instance of the module");
                    let run = instance.exports.get_function("main")?;
                    tracing::info!("Success: Retrieved the run function");
                    let result = {
                        let tmp = run.call(&mut self.store, &[])?;
                        tracing::info!("Success: Executed the program\n\t{:?}", &tmp);
                        tmp
                    };
                    self.chan.results.send(result).await?;
                }
                Some(transform) = self.chan.transform.recv() => {
                    self.surface.lock().unwrap().transform(transform);
                }
                _ = tokio::signal::ctrl_c() => {
                    break;
                }
                else => tracing::warn!("Tonic has no more work to do"),
            }
        })
    }
    pub fn spawn(self) -> tokio::task::JoinHandle<AsyncResult> {
        tokio::spawn(self.run())
    }
    pub fn surface(&self) -> &Arc<Mutex<Surface>> {
        &self.surface
    }
}