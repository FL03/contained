/*
    Appellation: tonic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A tonic is traditionally used to to describe the first degree (note) of the diatonic scale.
        Here, a tonic is used to describe a unit-computing machine that is capable of either executing a program or transforming its scope.
*/
use super::Surface;
use crate::neo::triads::LPR;
use scsys::prelude::AsyncResult;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use wasmer::{Instance, Module, Store};

#[derive(Debug)]
pub struct Tonic {
    program: mpsc::Receiver<Module>,
    store: Store,
    surface: Arc<Mutex<Surface>>,
    transform: mpsc::Receiver<LPR>,
}

impl Tonic {
    pub fn new(surface: Surface) -> Self {
        Self {
            program: mpsc::channel(9).1,
            store: Store::default(),
            surface: Arc::new(Mutex::new(surface)),
            transform: mpsc::channel(9).1,
        }
    }
    pub async fn run(mut self) -> AsyncResult {
        Ok(loop {
            tokio::select! {
                Some(module) = self.program.recv() => {
                    let host = self.surface.lock().unwrap().imports(&mut self.store);
                    let instance = Instance::new(&mut self.store, &module, &host).expect("Failed to instantiate module");
                    let run = instance.exports.get_function("run")?;

                    let result = run.call(&mut self.store, &[])?;
                    println!("{:?}", result);
                }
                Some(transform) = self.transform.recv() => {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neo::triads::*;

    #[test]
    fn test_tonic() {
        // Initialize a new triad
        let triad = Triad::new(0.into(), TriadClass::Major);
        // Initialize a new, stateful instance
        let instance = Surface::new(triad.clone());

        // Initialize a new tonic
        let tonic = Tonic::new(instance);
    }
}
