/*
    Appellation: tonnetz <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A tonnetz is a conceptual lattice diagram traditionally used for spatially representing tonal distance and relationships.

        For our purposes, we specifcally consider the generalized tonnetz in a graph-based form of interconnected octahedrals which create a circle tied together by interconneted faces.
        This means that each device will run at least one "cell" where a cell is a circular graph
*/
use crate::neo::{cmp::Note, Triad};
use crate::turing::{Configuration, Machine, Program, Symbolic, Tape};
use crate::{Resultant, State, States};
use serde::{Deserialize, Serialize};

pub fn triadic_machine(triad: Triad, program: Program<Note>) -> Resultant<Machine<Note>> {
    Machine::new(triad.root().clone(), program)
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Tonnetz {
    scope: Triad,
}

impl Tonnetz {
    pub fn new(scope: Triad) -> Self {
        Self { scope }
    }
    pub fn config(&self) -> Configuration<Note> {
        self.scope.clone().into()
    }
    pub fn machine(&self, program: Program<Note>) -> Resultant<Machine<Note>> {
        Machine::new(self.scope.root().clone(), program)
    }
}
