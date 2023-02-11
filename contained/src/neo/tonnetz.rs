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
use super::{LPR, Triad};
use crate::{cmp::Note, Resultant};
use crate::turing::{Machine, Program, Turing};
use serde::{Deserialize, Serialize};

pub fn triadic_machine(triad: Triad, program: Program<Note>) -> Resultant<Machine<Note>> {
    Machine::new(triad.root().clone(), program)
}

///
pub struct Conduit {
    fabric: Tonnetz,
    program: Program<Note>,
}

impl Turing for Conduit {
    type Symbol = Note;

    fn default_symbol(&self) -> &Self::Symbol {
        self.fabric.scope().root()
    }

    fn program(&self) -> &crate::turing::Program<Self::Symbol> {
        &self.program
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Tonnetz {
    scope: Triad,
}

impl Tonnetz {
    pub fn new(scope: Triad) -> Self {
        Self { scope }
    }
    /// Attempt to create a [Machine] with the given [Program] and active [Triad]
    pub fn machine(&self, program: Program<Note>) -> Resultant<Machine<Note>> {
        Machine::new(self.scope.root().clone(), program)
    }
    /// Returns an owned instance of the active [Triad]
    pub fn scope(&self) -> &Triad {
        &self.scope
    }
    /// Apply a single [LPR] transformation onto the active machine
    /// For convenience, [std::ops::Mul] was implemented as a means of applying the transformation
    pub fn transform(&mut self, shift: LPR) {
        self.scope = shift * self.scope().clone();
    }
    /// Applies multiple [LPR] transformations onto the scoped [Triad]
    /// The goal here is to allow the machine to work on and in the scope
    pub fn walk(&mut self, shifts: Vec<LPR>) {
        for s in shifts {
            self.transform(s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neo::{Triad, Triads};

    #[test]
    fn test_tonnetz() {
        let triad = Triad::build(0.into(), Triads::Major);

        let mut a = Tonnetz::new(triad.clone());
        // Apply three consecutive transformations to the scope
        a.walk(vec![LPR::L, LPR::P, LPR::R]);
        assert_eq!(a.scope().clone(), Triad::try_from((1, 4, 8)).unwrap());
        // Apply the same transformations in reverse to go back to the original
        a.walk(vec![LPR::R, LPR::P, LPR::L]);
        assert_eq!(a.scope().clone(), triad);
    }
}
