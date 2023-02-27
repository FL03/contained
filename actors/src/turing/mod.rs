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

use crate::{Resultant, States, Symbolic};
use scsys::prelude::Stateful;

pub trait Executable<T> {
    type Error;
    type Output;

    fn execute(&self, exe: &mut T) -> Result<Self::Output, Self::Error>;
    fn execute_once(&self, exe: &mut T) -> Result<Self::Output, Self::Error>;
    fn execute_until(&self, exe: &mut T, until: impl Fn(&T) -> bool) -> Result<Self::Output, Self::Error>;
}

/// Describes the basic functionality of a Turing machine
pub trait Turing {
    type Symbol: Symbolic;

    fn default_symbol(&self) -> &Self::Symbol;

    fn program(&self) -> &Program<Self::Symbol>;
    ///
    fn execute(
        &self,
        cnf: &mut Configuration<Self::Symbol>,
    ) -> Resultant<Configuration<Self::Symbol>> {
        self.execute_until(cnf, |cnf| cnf.state().clone().state() == States::invalid())
    }
    ///
    fn execute_once(
        &self,
        cnf: &mut Configuration<Self::Symbol>,
    ) -> Resultant<Configuration<Self::Symbol>> {
        let head = Head::new(cnf.state().clone(), cnf.symbol().clone());
        let inst = self.program().get(head)?.clone();
        cnf.state = inst.tail.state().clone();
        cnf.set_symbol(inst.tail.symbol().clone());
        cnf.shift(*inst.tail.action(), self.default_symbol().clone());
        Ok(cnf.clone())
    }
    ///
    fn execute_until(
        &self,
        cnf: &mut Configuration<Self::Symbol>,
        until: impl Fn(&Configuration<Self::Symbol>) -> bool,
    ) -> Resultant<Configuration<Self::Symbol>> {
        while !until(cnf) {
            let head = Head::new(cnf.state.clone(), cnf.symbol().clone());
            let inst = self.program().get(head)?.clone();
            cnf.state = inst.tail.state().clone();
            cnf.set_symbol(inst.tail.symbol().clone());
            cnf.shift(*inst.tail.action(), self.default_symbol().clone());
        }
        Ok(cnf.clone())
    }
    /// Translates and returns a mutated [`Tape`] using the [`TuringMachine::execute`]
    /// method as the [`Configuration::new_std`].
    fn translate_std(&self, tape: Tape<Self::Symbol>) -> Resultant<Tape<Self::Symbol>> {
        let mut conf = Configuration::std(tape)?;
        let exec = self.execute(&mut conf)?;
        Ok(exec.tape().clone())
    }

    /// Translates and returns a mutated [`Tape`] using the [`TuringMachine::execute`]
    /// method as the [`Configuration::new_nrm`].
    fn translate_nrm(&self, tape: Tape<Self::Symbol>) -> Resultant<Tape<Self::Symbol>> {
        let mut conf = Configuration::norm(tape)?;
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
