/*
    Appellation: machine <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{Configuration, Head, Program, Programatic, Symbolic, Turing};
use crate::Resultant;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Machine<S: Symbolic = String> {
    pub ds: S, // the default symbol
    pub program: Program<S>,
}

impl<S: Symbolic> Machine<S> {
    pub fn new(ds: S, program: Program<S>) -> Resultant<Self> {
        if program.alphabet().contains(&ds) {
            Ok(Self { ds, program })
        } else {
            return Err(format!(
                "The indicated default symbol ({}) is not present in the provided alphabet...",
                ds.to_string()
            ));
        }
    }
    pub fn initial(&self) -> S {
        self.ds.clone()
    }
}

impl<S: Symbolic> Turing for Machine<S> {
    type Symbol = S;

    fn execute_once(
        &self,
        cnf: &mut Configuration<Self::Symbol>,
    ) -> Resultant<Configuration<Self::Symbol>> {
        let head = Head::new(cnf.state(), cnf.symbol().expect("").clone());
        let inst = self.program.get(head)?.clone();
        cnf.state = inst.tail.state().clone();
        cnf.set_symbol(inst.tail.symbol().clone());
        cnf.shift(*inst.tail.action(), self.ds.clone());
        Ok(cnf.clone())
    }

    fn execute_until(
        &self,
        cnf: &mut Configuration<Self::Symbol>,
        until: impl Fn(&Configuration<Self::Symbol>) -> bool,
    ) -> Resultant<Configuration<Self::Symbol>> {
        while !until(cnf) {
            let head = Head::new(cnf.state.clone(), cnf.symbol().expect("").clone());
            let inst = self.program.get(head)?.clone();
            cnf.state = inst.tail.state().clone();
            cnf.set_symbol(inst.tail.symbol().clone());
            cnf.shift(*inst.tail.action(), self.ds.clone());
        }
        Ok(cnf.clone())
    }
}
