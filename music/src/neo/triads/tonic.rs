/*
    Appellation: tonic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A tonic is traditionally used to to describe the first degree (note) of the diatonic scale.
        For us, a tonic is used to describe the computable surface of a triad. The surface of the triad is described by its states while the triad itself is used
        to describe the subset of notes that are contained within the surface. That being said it is important to consider that each surface is better understood
        as the environment in-which a WASM machine is operating.
*/
use super::{Triad, Triadic};
use crate::{neo::LPR, MusicResult, Note};
use contained_core::states::{State, Stateful};
use contained_core::turing::Tape;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Default)]
pub struct Tonic {
    cursor: usize,
    memory: Arc<Mutex<Tape<Note>>>,
    triad: Arc<Mutex<Triad>>,
}

impl Tonic {
    pub fn current(&self) -> Note {
        self.memory.lock().unwrap()[self.cursor].clone()
    }
    pub fn run(&mut self) -> MusicResult<()> {
        while self.triad.lock().unwrap().state().is_valid() {
            if self.triad.lock().unwrap().contains(&self.current()) {
                self.memory
                    .lock()
                    .unwrap()
                    .write(self.cursor, self.current());
                self.cursor += 1;
            } else {
                // Invalidate the triad
                self.triad.lock().unwrap().state().invalidate();
                // TODO: Implement a means of transforming the triad without knowing the transformations required to get there
                // self.triad
                //     .lock()
                //     .unwrap()
                //     .transform_to_include(self.current());
            }
        }
        Ok(())
    }
}
