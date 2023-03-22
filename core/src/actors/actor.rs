/*
    Appellation: actor <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{Execute, Translate};
use crate::states::{State, Stateful};
use crate::turing::{instructions::Instruction, Program, Tape};
use crate::{Alphabet, Error, Scope, Symbolic};
use scsys::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Actor<S: Symbolic> {
    index: usize,
    pub memory: Tape<S>,
    program: Program<S>,
    state: State,
    ts: i64,
}

impl<S: Symbolic> Actor<S> {
    pub fn new(program: Program<S>, tape: Option<Tape<S>>) -> Self {
        Self {
            index: 0,
            memory: tape.unwrap_or_default(),
            program,
            state: Default::default(),
            ts: Timestamp::default().into(),
        }
    }

    pub fn insert_instruction(&mut self, instruction: Instruction<S>) -> Option<Instruction<S>> {
        self.program.insert(instruction)
    }
}

impl<S: Symbolic> Extend<S> for Actor<S> {
    fn extend<T: IntoIterator<Item = S>>(&mut self, iter: T) {
        self.memory.extend(iter)
    }
}

impl<S: Symbolic> Extend<Instruction<S>> for Actor<S> {
    fn extend<T: IntoIterator<Item = Instruction<S>>>(&mut self, iter: T) {
        self.program.extend(iter)
    }
}

impl<S: Symbolic> Iterator for Actor<S> {
    type Item = Instruction<S>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.clone().memory.get(self.index) {
            // Update the timestamp
            self.ts = Timestamp::default().into();
            // Get the instruction
            self.program.get((self.state, cur.clone()).into()).cloned()
        } else {
            None
        }
    }
}

impl<S: Symbolic> Alphabet<S> for Actor<S> {
    fn default_symbol(&self) -> S {
        self.program.default_symbol()
    }
}

impl<S: Symbolic> Execute<S> for Actor<S> {
    type Driver = Self;

    fn scope(&self) -> Self::Driver {
        self.clone()
    }

    fn scope_mut(&mut self) -> &mut Self::Driver {
        self
    }
}

impl<S: Symbolic> Scope<S> for Actor<S> {
    fn insert_symbol(&mut self, elem: S) {
        self.memory.insert(self.index(), elem);
    }

    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, pos: usize) {
        self.index = pos;
    }

    fn set_symbol(&mut self, elem: S) {
        self.memory.set(self.index(), elem);
    }

    fn tape(&self) -> &crate::turing::Tape<S> {
        &self.memory
    }
}

impl<S: Symbolic> Stateful for Actor<S> {
    type State = State;

    fn state(&self) -> Self::State {
        self.state
    }
    fn update_state(&mut self, state: Self::State) {
        self.state = state;
        self.ts = Timestamp::default().into();
    }
}

impl<S: Symbolic> Translate<S> for Actor<S> {
    fn translate(&mut self, tape: Tape<S>) -> Result<Tape<S>, Error> {
        *self = Self::new(self.program.clone(), Some(tape));
        Ok(self.memory.clone())
    }
}

impl<S: Symbolic> From<Program<S>> for Actor<S> {
    fn from(program: Program<S>) -> Self {
        Self::new(program, None)
    }
}

impl<S: Symbolic> From<Tape<S>> for Actor<S> {
    fn from(tape: Tape<S>) -> Self {
        Self::new(Program::default(), Some(tape))
    }
}
