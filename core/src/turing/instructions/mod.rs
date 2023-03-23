/*
    Appellation: instructions <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        Turing machines accept instructions in the form of a five-tuple:
            (State, Symbol, State, Symbol, Move)
*/
pub use self::{head::*, instruction::*, moves::*, tail::*};

pub(crate) mod head;
pub(crate) mod instruction;
pub(crate) mod moves;
pub(crate) mod tail;

use crate::states::{State, Stateful};
use crate::turing::Symbolic;

pub trait IHead<S: Symbolic>: Stateful<State> {
    fn symbol(&self) -> S;
}

pub trait ITail<S: Symbolic>: Stateful<State> {
    fn action(&self) -> Move;
    fn symbol(&self) -> S;
}

pub trait InstructionSpec<S: Symbolic>: IntoIterator<Item = Self> {
    type Head: IHead<S>;
    type Tail: ITail<S>;

    fn new(head: Self::Head, tail: Self::Tail) -> Self;
    fn head(&self) -> Self::Head;
    fn tail(&self) -> Self::Tail;
}

pub trait InstructionSet<S: Symbolic>: Iterator<Item = Instruction<S>> {
    type Head: IHead<S>;
    type Tail: ITail<S>;

    fn new(head: Self::Head, tail: Self::Tail) -> Self;
    fn cursor(&self) -> usize;
}
