/*
    Appellation: turing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{operator::*, instructions::*, machine::*, programs::*, tapes::*};

pub(crate) mod operator;
pub(crate) mod instructions;
pub(crate) mod machine;
pub(crate) mod programs;
pub(crate) mod tapes;

use crate::{Scope, Stateful, States};

pub trait Alphabet<S: Symbolic>: Clone + std::iter::IntoIterator<Item = S> {
    fn alphabet(&self) -> Vec<S> {
        Vec::from_iter(self.clone())
    }
}

impl<S: Symbolic> Alphabet<S> for Vec<S> {}

/// Simple trait for compatible symbols
pub trait Symbolic:
    Clone
    + Default
    + Eq
    + Ord
    + PartialEq
    + PartialOrd
    + std::fmt::Debug
    + std::fmt::Display
    + serde::Serialize
{
}

impl Symbolic for char {}

impl Symbolic for &str {}

impl Symbolic for String {}

/// [Turing] describes a programmable Turing machine
pub trait Turing<S: Symbolic> {
    type Scope: Clone + Scope<S>;

    fn driver(&mut self) -> &mut Self::Scope;
    ///
    fn execute(&mut self, program: Program<S>) -> Result<Self::Scope, String> {
        let until = |actor: &Self::Scope| actor.state().state().clone() == States::Invalid;
        self.execute_until(program, until)
    }
    ///
    fn execute_once(&mut self, program: Program<S>) -> Result<Self::Scope, String> {
        let head = Head::new(
            self.driver().state().clone().into(),
            self.driver().scope().clone(),
        );
        let inst = program.get(head)?.clone();
        self.driver().set_state(inst.tail().state().clone());
        self.driver().set_symbol(inst.tail().symbol().clone());
        self.driver()
            .shift(*inst.tail().action(), program.default_symbol().clone());
        Ok(self.driver().clone())
    }
    ///
    fn execute_until(
        &mut self,
        program: Program<S>,
        until: impl Fn(&Self::Scope) -> bool,
    ) -> Result<Self::Scope, String> {
        while !until(self.driver()) {
            let head = Head::new(
                self.driver().state().clone().into(),
                self.driver().scope().clone(),
            );
            let inst = program.get(head)?.clone();
            self.driver().set_state(inst.tail().state().clone());
            self.driver().set_symbol(inst.tail().symbol().clone());
            self.driver()
                .shift(*inst.tail().action(), program.default_symbol().clone());
        }
        Ok(self.driver().clone())
    }
    fn translate(&mut self, program: Program<S>, tape: Tapes<S>) -> Result<Tape<S>, String> {
        self.update(Self::Scope::build(tape));
        let exec = self.execute(program)?;
        Ok(exec.tape().clone())
    }
    fn update(&mut self, scope: Self::Scope);
}

/// [With] describes a simple means of concating several objects together
pub trait With<T> {
    /// [With::Output] must be a superposition of self and T
    type Output;

    /// [With::with] accepts an owned instance of the given type and returns a [With::Output] instance
    fn with(&self, other: &T) -> Self::Output;
}
