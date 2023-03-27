/*
    Appellation: exec <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::turing::{instructions::Instruction, Program};
use crate::{Executable, Symbolic};

pub struct Executor<S: Symbolic> {
    program: Program<S>,
}

impl<S: Symbolic> Executor<S> {
    pub fn new(program: Program<S>) -> Self {
        Self { program }
    }
    pub fn program(&self) -> Program<S> {
        self.program.clone()
    }
}

impl<S: Symbolic> Extend<Instruction<S>> for Executor<S> {
    fn extend<T: IntoIterator<Item = Instruction<S>>>(&mut self, iter: T) {
        self.program.extend(iter)
    }
}

// impl<S: Symbolic> Iterator for Executor<S> {
//     type Item = Instruction<S>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.program.next()
//     }
// }
