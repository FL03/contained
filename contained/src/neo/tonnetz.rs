/*
    Appellation: tonnetz <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A tonnetz is a conceptual lattice diagram traditionally used for spatially representing tonal distance and relationships.

        For our purposes, we specifcally consider the generalized tonnetz in a graph-based form of interconnected octahedrals which create a circle tied together by interconneted faces.
        This means that each device will run at least one "cell" where a cell is a circular graph

        This provides that the tonnetz is some sort of zeno-machine as each compute surface is capable of executing a countably infinite amount of steps....
        Another option being considered is the multiway turing machine
*/
use crate::neo::{cmp::Note, LPR, Triad, Triadic};
use crate::turing::{Configuration, Machine, Program};
use crate::Resultant;
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
    pub fn scope(&self) -> &Triad {
        &self.scope
    }
    pub fn transform(&mut self, shift: LPR) {
        self.scope = shift.transform(&mut self.scope.clone());
    }
}
