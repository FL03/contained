/*
    Appellation: tonic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A tonic is traditionally used to to describe the first degree (note) of the diatonic scale.
        For us, a tonic is used to describe the computable surface of a triad. The surface of the triad is described by its states while the triad itself is used
        to describe the subset of notes that are contained within the surface. That being said it is important to consider that each surface is better understood
        as the environment in-which a WASM machine is operating.
*/
use super::Surface;
use crate::neo::triads::LPR;
use scsys::prelude::AsyncResult;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use wasmer::{Instance, Module, Store};

#[derive(Debug)]
pub struct Tonic {
    memory: Store,

    program: mpsc::Receiver<Module>,
    surface: Arc<Mutex<Surface>>,
    transform: mpsc::Receiver<LPR>
}

impl Tonic {
    pub fn new(surface: Surface) -> Self {
        Self {
            memory: Store::default(),
            program: mpsc::channel(9).1,
            surface: Arc::new(Mutex::new(surface)),
            transform: mpsc::channel(9).1,
        }
    }
    pub async fn run(mut self) -> AsyncResult {
        Ok(loop {
            tokio::select! {
                Some(module) = self.program.recv() => {
                    let host = self.surface.lock().unwrap().imports(&mut self.memory);
                    let instance = Instance::new(&mut self.memory, &module, &host).expect("Failed to instantiate module");
                    let run = instance.exports.get_function("run")?;

                    let result = run.call(&mut self.memory, &[])?;
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
        tokio::spawn( self.run() )
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
