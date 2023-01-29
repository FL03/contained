/*
    Appellation: programs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{Instruction, Head, Symbolic};
use crate::State;

pub struct Program<S: Symbolic> {
    pub alphabet: Vec<S>,
    pub instructions: Vec<Instruction<S>>,
    pub final_state: State
}

impl<S: Symbolic> Program<S> {
    pub fn new(alphabet: Vec<S>, final_state: State) -> Self {
        let s = final_state.clone().state as i64;
        let capacity = alphabet.clone().len() * s as usize;
        let instructions = Vec::with_capacity(capacity);

        Self { alphabet, instructions, final_state }
    }
    pub fn alphabet(&self) -> &Vec<S> {
        &self.alphabet
    }
    pub fn instructions(&self) -> &Vec<Instruction<S>> {
        &self.instructions
    }
    pub fn final_state(&self) -> &State {
        &self.final_state
    }
    pub fn get(&self, head: Head<S>) -> Result<Option<&Instruction<S>>, Box<dyn std::error::Error>> {
        if self.final_state().state > head.state().state {
            panic!("")
        } else {
            let res = self.instructions().iter()
            .find(|inst: &&Instruction<S>| &inst.head == &head);

            Ok(res)
        }
    }
}