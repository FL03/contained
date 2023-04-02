/*
    Appellation: tonic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A tonic is traditionally used to to describe the first degree (note) of the diatonic scale.
        Here, a tonic is used to describe a unit-computing machine that is capable of either executing a program or transforming its scope.
*/
use super::{Surface, Triad, LPR};
use scsys::prelude::AsyncResult;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use wasmer::{Instance, Module, Store};

#[derive(Debug)]
pub struct TonicChannel {
    pub program: mpsc::Receiver<Module>,
    pub transform: mpsc::Receiver<LPR>,
}

impl TonicChannel {
    pub fn new(program: mpsc::Receiver<Module>, transform: mpsc::Receiver<LPR>) -> Self {
        Self { program, transform }
    }
    pub fn program(&self) -> &mpsc::Receiver<Module> {
        &self.program
    }
    pub fn transform(&self) -> &mpsc::Receiver<LPR> {
        &self.transform
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            program: mpsc::channel(capacity).1,
            transform: mpsc::channel(capacity).1,
        }
    }
}

impl Default for TonicChannel {
    fn default() -> Self {
        Self::with_capacity(9)
    }
}

#[derive(Debug)]
pub struct Tonic {
    pub cache: Arc<Mutex<Vec<Box<[wasmer::Value]>>>>,
    chan: TonicChannel,
    store: Store,
    surface: Arc<Mutex<Surface>>,
}

impl Tonic {
    pub fn new(chan: TonicChannel, scope: Triad) -> Self {
        Self {
            cache: Arc::new(Mutex::new(Vec::new())),
            chan,
            store: Store::default(),
            surface: Arc::new(Mutex::new(Surface::new(scope))),
        }
    }
    pub fn cache(&self) -> &Arc<Mutex<Vec<Box<[wasmer::Value]>>>> {
        &self.cache
    }
    pub fn depth(&self) -> usize {
        self.cache.lock().unwrap().len()
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
                    self.cache.lock().unwrap().push(result);
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
