/*
    Appellation: turing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{cells::*, instructions::*, machine::*, programs::*};

pub(crate) mod cells;
pub(crate) mod instructions;
pub(crate) mod machine;
pub(crate) mod programs;

pub type Dirac<S, T> = dyn Fn(S) -> T;

pub type Tape = Vec<Cell>;

pub trait Symbolic: Clone + Default + PartialEq + ToString {} 

pub trait Transition<S: Clone> {
    type Output;

    fn data(&self) -> &S;
    fn dirac(&self) -> &Dirac<S, Self::Output>;

    fn resultant(&self) -> Self::Output {
        self.dirac()(self.data().clone())
    }
}