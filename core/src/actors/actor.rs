/*
    Appellation: actor <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::states::{State, Stateful};
use crate::turing::{
    instructions::{Instruction, Move},
    Program, Tape, Turing,
};
use crate::{Resultant, Scope, Symbolic};
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
    pub fn new(program: Program<S>) -> Self {
        Self {
            index: 0,
            memory: Tape::default(),
            program,
            state: Default::default(),
            ts: Timestamp::default().into(),
        }
    }
    pub fn insert_instruction(
        &mut self,
        instruction: Instruction<S>,
    ) -> Resultant<Option<Instruction<S>>> {
        self.program.insert(instruction)
    }
}

impl<S: Symbolic> Iterator for Actor<S> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.clone().memory.get(self.index) {
            // Update the timestamp
            self.ts = Timestamp::default().into();
            // Get the instruction
            let _ = match self.clone().program.get((self.state, cur.clone()).into()) {
                Ok(instruction) => {
                    self.state = instruction.clone().tail().state();
                    self.memory
                        .set(self.index(), instruction.clone().tail().symbol());
                    self.shift(
                        instruction.clone().tail().action(),
                        self.program.clone().default_symbol(),
                    );
                }
                Err(_) => {}
            };
            Some(cur.clone())
        } else {
            None
        }
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

impl<S: Symbolic> Scope<S> for Actor<S> {
    fn insert(&mut self, elem: S) {
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

impl<S: Symbolic> Turing<S> for Actor<S> {
    type Error = Box<dyn std::error::Error>;
    type Scope = Self;

    fn execute(&mut self) -> Result<&Self, Self::Error> {
        self.execute_until(|s: &Self::Scope| s.state() == State::invalid())
    }

    fn execute_once(&mut self) -> Result<&Self, Self::Error> {
        if let Some(cur) = self.clone().memory.get(self.index) {
            // Update the timestamp
            self.ts = Timestamp::default().into();
            // Get the instruction
            let _ = match self.clone().program.get((self.state, cur.clone()).into()) {
                Ok(instruction) => {
                    self.state = instruction.clone().tail().state();
                    self.memory
                        .set(self.index(), instruction.clone().tail().symbol());
                    self.shift(
                        instruction.clone().tail().action(),
                        self.program.clone().default_symbol(),
                    );
                }
                Err(_) => {}
            };
            Ok(self)
        } else {
            Err("No more instructions to execute".into())
        }
    }

    fn execute_until(
        &mut self,
        until: impl Fn(&Self::Scope) -> bool,
    ) -> Result<&Self, Self::Error> {
        while !until(self) {
            self.execute_once()?;
        }
        Ok(self)
    }

    fn translate(&mut self, tape: Tape<S>) -> Result<Tape<S>, Self::Error> {
        todo!()
    }
}
