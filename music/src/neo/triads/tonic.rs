/*
    Appellation: tonic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A tonic is traditionally used to to describe the first degree (note) of the diatonic scale.
        For us, a tonic is used to describe the computable surface of a triad. The surface of the triad is described by its states while the triad itself is used
        to describe the subset of notes that are contained within the surface. That being said it is important to consider that each surface is better understood
        as the environment in-which a WASM machine is operating.
*/
use super::{Instance, Triad, Triadic};
use crate::{MusicResult, Note};
use contained_core::turing::{instructions::Instruction, Program, Tape};
use contained_core::{Scope, State, Stateful, ArrayLike, Include, Insert};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Default)]
pub struct Tonic {
    cursor: usize,
    memory: Arc<Mutex<Tape<Note>>>,
    program: Program<Note>,
    surface: Arc<Mutex<Instance>>,
}

impl Tonic {
    pub fn current(&self) -> Note {
        self.memory.lock().unwrap()[self.cursor].clone()
    }
    pub fn run(&mut self) -> MusicResult<()> {
        while self.surface.lock().unwrap().state().is_valid() {
            if self
                .surface
                .lock()
                .unwrap()
                .triad()
                .contains(&self.current())
            {
                self.memory
                    .lock()
                    .unwrap()
                    .write(self.cursor, self.current());
                self.cursor += 1;
            } else {
                // Invalidate the triad
                self.surface.lock().unwrap().state().invalidate();
                // Find a path to the current note
                let path = self
                    .surface
                    .lock()
                    .unwrap()
                    .triad()
                    .pathfinder(self.current())
                    .find();
                // If a path is found, walk it
                if let Some(path) = path {
                    self.surface.lock().unwrap().triad().walk(path);
                    // Validate the triad
                    self.surface.lock().unwrap().state().validate();
                } else {
                    return Err("No path found".into());
                }
            }
        }
        Ok(())
    }
    pub fn surface(&self) -> &Arc<Mutex<Instance>> {
        &self.surface
    }
}

impl Extend<Note> for Tonic {
    fn extend<I: IntoIterator<Item = Note>>(&mut self, iter: I) {
        self.memory.lock().unwrap().extend(iter);
    }
}

impl Extend<Instruction<Note>> for Tonic {
    fn extend<I: IntoIterator<Item = Instruction<Note>>>(&mut self, iter: I) {
        self.program.extend(iter);
    }
}

impl Include<Note> for Tonic {
    fn include(&mut self, elem: Note) {
        self.memory.lock().unwrap().insert(self.cursor, elem);
    }
}

impl Include<Instruction<Note>> for Tonic {
    fn include(&mut self, elem: Instruction<Note>) {
        self.program.include(elem);
    }
}

impl Insert<usize, Note> for Tonic {
    fn insert(&mut self, index: usize, elem: Note) {
        self.memory.lock().unwrap().insert(index, elem);
    }
}

impl Iterator for Tonic {
    type Item = Instruction<Note>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.clone().memory.lock().unwrap().get(self.cursor) {
            // Get the instruction
            self.program.get((self.state(), cur.clone()).into()).cloned()
        } else {
            None
        }
    }
}

impl Stateful<State> for Tonic {
    fn state(&self) -> State {
        self.surface.lock().unwrap().state()
    }

    fn update_state(&mut self, state: State) {
        self.surface.lock().unwrap().update_state(state)
    }
}
