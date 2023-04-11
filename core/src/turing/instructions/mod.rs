/*
    Appellation: instructions <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Turing machines accept instructions in the form of a five-tuple:
            (State, Symbol, State, Symbol, Move)
*/
pub use self::{head::*, instruction::*, iter::*, moves::*, tail::*};

mod head;
mod instruction;
mod iter;
mod moves;
mod tail;

use super::{State, Symbolic};
use crate::states::Stateful;

pub trait InstructionHead<S: Symbolic>: Stateful<State> {
    fn symbol(&self) -> S;
}

pub trait InstructionTail<S: Symbolic>: Stateful<State> {
    fn action(&self) -> Move;
    fn symbol(&self) -> S;
}

pub trait InstructionSpec<S: Symbolic>: IntoIterator<Item = Self> {
    type Head: InstructionHead<S>;
    type Tail: InstructionTail<S>;

    fn new(head: Self::Head, tail: Self::Tail) -> Self;
    fn head(&self) -> Self::Head;
    fn tail(&self) -> Self::Tail;
}

pub trait InstructionSet<S: Symbolic>: Iterator<Item = Instruction<S>> {
    type Head: InstructionHead<S>;
    type Tail: InstructionTail<S>;

    fn new(head: Self::Head, tail: Self::Tail) -> Self;
    fn cursor(&self) -> usize;
}
