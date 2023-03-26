/*
    Appellation: tonic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A tonic is traditionally used to to describe the first degree (note) of the diatonic scale.
        For us, a tonic is used to describe the computable surface of a triad. The surface of the triad is described by its states while the triad itself is used
        to describe the subset of notes that are contained within the surface. That being said it is important to consider that each surface is better understood
        as the environment in-which a WASM machine is operating.
*/
use super::{Instance, Triadic};
use crate::{MusicResult, Note};
use contained_core::turing::{instructions::Instruction, Program, Tape};
use contained_core::{
    actors::Execute, Alphabet, ArrayLike, Include, Insert, Scope, State, Stateful,
};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Default)]
pub struct Tonic {
    cursor: usize,
    memory: Arc<Mutex<Tape<Note>>>,
    program: Program<Note>,
    surface: Arc<Mutex<Instance>>,
}

impl Tonic {
    pub fn new(program: Program<Note>, surface: Instance) -> Self {
        Self {
            cursor: 0,
            memory: Arc::new(Mutex::new(Tape::new())),
            program,
            surface: Arc::new(Mutex::new(surface)),
        }
    }
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
                self.execute()?;
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
                    self.execute()?;
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

impl Alphabet<Note> for Tonic {
    fn is_viable(&self, symbol: &Note) -> bool {
        self.program.is_viable(symbol)
    }
    fn default_symbol(&self) -> Note {
        self.surface.lock().unwrap().triad().root()
    }
}

impl Execute<Note> for Tonic {
    type Driver = Self;

    fn scope(&self) -> &Self::Driver {
        self
    }

    fn scope_mut(&mut self) -> &mut Self::Driver {
        self
    }

    fn program(&self) -> &Program<Note> {
        &self.program
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
            if !self.surface.lock().unwrap().triad().contains(cur) {
                // Invalidate the triad
                self.surface.lock().unwrap().state().invalidate();
                // Find a path to the current note
                let path = self
                    .surface
                    .lock()
                    .unwrap()
                    .triad()
                    .pathfinder(cur.clone())
                    .find();
                // If a path is found, walk it
                if let Some(path) = path {
                    self.surface.lock().unwrap().triad().walk(path);
                    // Validate the triad
                    self.surface.lock().unwrap().state().validate();
                } else {
                    return None;
                }
            }
            // Get the instruction
            self.program
                .get((self.state(), cur.clone()).into())
                .cloned()
        } else {
            None
        }
    }
}

impl Scope<Note> for Tonic {
    fn cursor(&self) -> usize {
        self.cursor
    }

    fn set_index(&mut self, pos: usize) {
        self.cursor = pos;
    }

    fn set_symbol(&mut self, elem: Note) {
        self.memory.lock().unwrap().set(self.cursor(), elem);
    }

    fn tape(&self) -> Tape<Note> {
        self.memory.lock().unwrap().clone()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neo::triads::*;

    // Test alphabet; allows programs to be written leveraging the complete 12 note alphabet
    const TEST_ALPHABET: [i64; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

    #[test]
    fn test_tonic() {
        // Initialize a new triad
        let triad = Triad::new(0.into(), Triads::Major);
        // Initialize a new, stateful instance
        let instance = Instance::new(triad.clone());
        // Initialize a new tape
        let tape: Tape<Note> = Tape::norm([0.into(), 1.into(), 3.into()]);

        // Instruction set; turn ["a", "b", "c"] into ["c", "a", "a"]
        let instructions: Vec<Instruction<Note>> = vec![
            (
                State::default(),
                0.into(),
                State::default(),
                7.into(),
                1.into(),
            )
                .into(),
            (
                State::default(),
                1.into(),
                State::default(),
                4.into(),
                1.into(),
            )
                .into(),
            (
                State::default(),
                3.into(),
                State::invalid(),
                0.into(),
                0.into(),
            )
                .into(),
        ];
        // Setup the program
        let program = Program::new(
            TEST_ALPHABET.iter().map(Note::from).collect::<Vec<_>>(),
            State::invalid(),
        );
        // Initialize a new machine
        let mut tonic = Tonic::new(program, instance);
        // Extend the machine memory; insert [0, 1, 4] into the tape
        tonic.extend(tape);
        // Extend the program; turn [0, 1, 4] into [7, 4, 0]
        tonic.extend(instructions);
        // Execute the program
        assert!(tonic.execute().is_ok());
        // Assert the result
        assert_eq!(tonic.tape(), Tape::norm([7.into(), 4.into(), 0.into()]));
        // Assert that the current triad is not the same as the original
        // assert_ne!(tonic.surface().lock().unwrap().triad(), triad);
    }
}
