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

use crate::{Operator, Resultant, Scope, Stateful, States};

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

pub trait Alphabet<S: Symbolic>: Clone + std::iter::IntoIterator<Item = S> {
    fn alphabet(&self) -> Vec<S> {
        Vec::from_iter(self.clone())
    }
}

impl<S: Symbolic> Alphabet<S> for Vec<S> {}

pub trait Execute<S: Symbolic> {
    type Scope: Clone + Scope<S>;
    type Error;

    fn driver(&mut self) -> &mut Self::Scope;
    ///
    fn execute(&mut self, program: Program<S>) -> Result<Self::Scope, String> {
        let until = | actor: &Self::Scope | actor.state().state().clone() == States::Invalid;
        self.execute_until( program, until)
    }
    ///
    fn execute_once(&mut self, program: Program<S>) -> Result<Self::Scope, String> {
        let head = Head::new(self.driver().state().clone().into(), self.driver().scope().clone());
        let inst = program.get(head)?.clone();
        self.driver().set_state(inst.tail().state().clone());
        self.driver().set_symbol(inst.tail().symbol().clone());
        self.driver().shift(*inst.tail().action(), program.default_symbol().clone());
        Ok(self.driver().clone())
    }
    ///
    fn execute_until(
        &mut self,
        program: Program<S>,
        until: impl Fn(&Self::Scope) -> bool,
    ) -> Result<Self::Scope, String> {
        while !until(self.driver()) {
            let head = Head::new(self.driver().state().clone().into(), self.driver().scope().clone());
            let inst = program.get(head)?.clone();
            self.driver().set_state(inst.tail().state().clone());
            self.driver().set_symbol(inst.tail().symbol().clone());
            self.driver().shift(*inst.tail().action(), program.default_symbol().clone());
        }
        Ok(self.driver().clone())
    }
}

/// Describes the basic functionality of a Turing machine
pub trait Turing<S: Symbolic> {
    fn default_symbol(&self) -> &S {
        self.program().default_symbol()
    }

    fn program(&self) -> &Program<S>;
    ///
    fn execute(&self, cnf: &mut Configuration<S>) -> Resultant<Configuration<S>> {
        self.execute_until(cnf, |cnf| cnf.state().state().clone() == States::invalid())
    }
    ///
    fn execute_once(&self, cnf: &mut Configuration<S>) -> Resultant<Configuration<S>> {
        let head = Head::new(cnf.state().clone().into(), cnf.scope().clone());
        let inst = self.program().get(head)?.clone();
        cnf.set_state(inst.tail().state().clone());
        cnf.set_symbol(inst.tail().symbol().clone());
        cnf.shift(*inst.tail().action(), self.default_symbol().clone());
        Ok(cnf.clone())
    }
    ///
    fn execute_until(
        &self,
        cnf: &mut Configuration<S>,
        until: impl Fn(&Configuration<S>) -> bool,
    ) -> Resultant<Configuration<S>> {
        while !until(cnf) {
            let head = Head::new(cnf.state.clone(), cnf.scope().clone());
            let inst = self.program().get(head)?.clone();
            cnf.set_state(inst.tail().state().clone());
            cnf.set_symbol(inst.tail().symbol().clone());
            cnf.shift(*inst.tail().action(), self.default_symbol().clone());
        }
        Ok(cnf.clone())
    }
    /// Translates and returns a mutated [`Tape`] using the [`TuringMachine::execute`]
    /// method as the [`Configuration::new_std`].
    fn translate_std(&self, tape: Tape<S>) -> Resultant<Tape<S>> {
        let mut conf = Configuration::build(tape, Some(Config::Standard));
        let exec = self.execute(&mut conf)?;
        Ok(exec.tape().clone())
    }

    /// Translates and returns a mutated [`Tape`] using the [`TuringMachine::execute`]
    /// method as the [`Configuration::new_nrm`].
    fn translate_nrm(&self, tape: Tape<S>) -> Resultant<Tape<S>> {
        let mut conf = Configuration::build(tape, Some(Config::Normal));
        let exec = self.execute(&mut conf)?;
        Ok(exec.tape().clone())
    }
}

/// [With] describes a simple means of concating several objects together
pub trait With<T> {
    /// [With::Output] must be a superposition of self and T
    type Output;

    /// [With::with] accepts an owned instance of the given type and returns a [With::Output] instance
    fn with(&self, other: &T) -> Self::Output;
}
