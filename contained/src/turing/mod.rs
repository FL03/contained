/*
    Appellation: turing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{configuration::*, instructions::*, machine::*, programs::*, tapes::*};

pub(crate) mod configuration;
pub(crate) mod instructions;
pub(crate) mod machine;
pub(crate) mod programs;
pub(crate) mod tapes;

pub type Dirac<S, T> = dyn Fn(S) -> T;

pub trait Symbolic: Clone + Default + Eq + PartialEq + ToString {}

impl Symbolic for char {}

impl Symbolic for &str {}

impl Symbolic for String {}

pub trait Transition<S: Clone> {
    type Output;

    fn data(&self) -> &S;
    fn dirac(&self) -> &Dirac<S, Self::Output>;
    fn resultant(&self) -> Self::Output {
        self.dirac()(self.data().clone())
    }
}
