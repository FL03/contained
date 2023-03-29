/*
    Appellation: tonic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A tonic is traditionally used to to describe the first degree (note) of the diatonic scale.
        For us, a tonic is used to describe the computable surface of a triad. The surface of the triad is described by its states while the triad itself is used
        to describe the subset of notes that are contained within the surface. That being said it is important to consider that each surface is better understood
        as the environment in-which a WASM machine is operating.
*/
use super::Surface;

use std::sync::{Arc, Mutex};
use wasmer::{Module, Store};

#[derive(Debug)]
pub struct Tonic {
    memory: Store,
    program: Module,
    surface: Arc<Mutex<Surface>>,
}

impl Tonic {
    pub fn new(program: Module, surface: Surface) -> Self {
        Self {
            memory: Store::default(),
            program,
            surface: Arc::new(Mutex::new(surface)),
        }
    }
    // pub fn run(&mut self) -> MusicResult<()> {
    //     while self.surface.lock().unwrap().state().is_valid() {
    //         if self
    //             .surface
    //             .lock()
    //             .unwrap()
    //             .triad()
    //             .contains(&self.current())
    //         {

    //         } else {
    //             // Invalidate the triad
    //             self.surface.lock().unwrap().state().invalidate();
    //             // Find a path to the current note
    //             let path = self
    //                 .surface
    //                 .lock()
    //                 .unwrap()
    //                 .triad()
    //                 .pathfinder(self.current())
    //                 .find();
    //             // If a path is found, walk it
    //             if let Some(path) = path {
    //                 self.surface.lock().unwrap().triad().walk(path);
    //                 // Validate the triad
    //                 self.surface.lock().unwrap().state().validate();
    //             } else {
    //                 return Err("No path found".into());
    //             }
    //         }
    //     }
    //     Ok(())
    // }
    pub fn surface(&self) -> &Arc<Mutex<Surface>> {
        &self.surface
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neo::triads::*;

    // Test alphabet; allows programs to be written leveraging the complete 12 note alphabet
    const TEST_ALPHABET: [i64; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

    #[test]
    fn test_tonic() {
        // Initialize a new triad
        let triad = Triad::new(0.into(), TriadClass::Major);
        // Initialize a new, stateful instance
        let instance = Surface::new(triad.clone());
    }
}
