/*
    Appellation: actor <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::Execute;
use crate::turing::{instructions::Instruction, Alphabet, Program, Symbolic, Tape};
use crate::{ArrayLike, Error, Include, Insert, Scope, State, Stateful, Translate};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Actor<S: Symbolic> {
    cursor: usize,
    pub memory: Tape<S>,
    program: Program<S>,
    state: State,
}

impl<S: Symbolic> Actor<S> {
    pub fn new(program: Program<S>, memory: Option<Tape<S>>) -> Self {
        Self {
            cursor: 0,
            memory: memory.unwrap_or_default(),
            program,
            state: Default::default(),
        }
    }
}

impl<S: Symbolic> Alphabet<S> for Actor<S> {
    fn is_viable(&self, symbol: &S) -> bool {
        self.program.is_viable(symbol)
    }
    fn default_symbol(&self) -> S {
        self.program.default_symbol()
    }
}

impl<S: Symbolic> AsMut<Actor<S>> for Actor<S> {
    fn as_mut(&mut self) -> &mut Actor<S> {
        self
    }
}

impl<S: Symbolic> AsRef<Actor<S>> for Actor<S> {
    fn as_ref(&self) -> &Actor<S> {
        self
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

impl<S: Symbolic> Include<S> for Actor<S> {
    fn include(&mut self, elem: S) {
        self.memory.insert(self.cursor, elem);
    }
}

impl<S: Symbolic> Insert<usize, S> for Actor<S> {
    fn insert(&mut self, index: usize, elem: S) {
        self.memory.insert(index, elem);
    }
}

impl<S: Symbolic> Include<Instruction<S>> for Actor<S> {
    fn include(&mut self, elem: Instruction<S>) {
        self.program.insert(elem);
    }
}

impl<S: Symbolic> Iterator for Actor<S> {
    type Item = Instruction<S>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.clone().memory.get(self.cursor) {
            // Get the instruction
            self.program.get((self.state, cur.clone()).into()).cloned()
        } else {
            None
        }
    }
}

impl<S: Symbolic> Execute<S> for Actor<S> {
    type Driver = Self;

    fn scope(&self) -> &Self::Driver {
        self
    }

    fn scope_mut(&mut self) -> &mut Self::Driver {
        self
    }

    fn program(&self) -> &Program<S> {
        &self.program
    }
}

impl<S: Symbolic> Scope<S> for Actor<S> {
    fn cursor(&self) -> usize {
        self.cursor
    }

    fn set_index(&mut self, pos: usize) {
        self.cursor = pos;
    }

    fn set_symbol(&mut self, elem: S) {
        self.memory.set(self.cursor(), elem);
    }

    fn tape(&self) -> Tape<S> {
        self.memory.clone()
    }
}

impl<S: Symbolic> Stateful<State> for Actor<S> {
    fn state(&self) -> State {
        self.state
    }

    fn update_state(&mut self, state: State) {
        self.state = state;
    }
}

impl<S: Symbolic> Translate<S> for Actor<S> {
    type Error = Error;

    fn translate(&mut self, tape: Tape<S>) -> Result<Tape<S>, Self::Error> {
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
