/*
    Appellation: machine <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::turing::{Program, Turing};
use crate::{Resultant, Symbolic};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Machine<S: Symbolic = String> {
    ds: S, // the default symbol
    program: Program<S>,
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
    pub fn is_valid(&self) -> bool {
        if self.program.alphabet().contains(&self.ds) {
            return true;
        }
        false
    }
}

impl<S: Symbolic> Turing for Machine<S> {
    type Symbol = S;

    fn default_symbol(&self) -> &S {
        &self.ds
    }

    fn program(&self) -> &Program<Self::Symbol> {
        &self.program
    }
}
