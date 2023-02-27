/*
    Appellation: tonnetz <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A tonnetz is a conceptual lattice diagram traditionally used for spatially representing tonal distance and relationships.

        For our purposes, we specifcally consider the generalized tonnetz in a graph-based form of interconnected octahedrals which create a circle tied together by interconneted faces.
        This means that each device will run at least one "cell" where a cell is a circular graph

        This provides that the tonnetz is some sort of zeno-machine as each compute surface is capable of executing a countably infinite amount of steps....
        Another option being considered is the multiway turing machine

        If we consider a single triad to be the scope of a single tonnetz, than we can consider a single tonnetz to be a persistant set of non-repeating traidic structures.
        Any two triads are connected if they share two notes or a single edge.
*/
use super::{Triad, LPR};
use crate::core::{Notable, Note};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, SmartDefault)]
pub struct Tonnetz<N: Notable = Note> {
    scope: Triad<N>,
}

impl<N: Notable> Tonnetz<N> {
    pub fn new(scope: Triad<N>) -> Self {
        Self { scope }
    }
    /// Returns an owned instance of the active [Triad]
    pub fn scope(&self) -> &Triad<N> {
        &self.scope
    }
    /// Apply a single [LPR] transformation onto the active machine
    /// For convenience, [std::ops::Mul] was implemented as a means of applying the transformation
    pub fn transform(&mut self, shift: LPR) {
        self.scope = shift * self.scope().clone();
    }
    /// Applies multiple [LPR] transformations onto the scoped [Triad]
    /// The goal here is to allow the machine to work on and in the scope
    pub fn walk(&mut self, cycle: Vec<LPR>) {
        for s in cycle {
            self.transform(s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Note;
    use crate::neo::{Triad, Triads};

    #[test]
    fn test_tonnetz() {
        let triad = Triad::<Note>::new(0.into(), Triads::Major);

        let mut a = Tonnetz::new(triad.clone());
        // Apply three consecutive transformations to the scope
        a.walk(vec![LPR::L, LPR::P, LPR::R]);
        assert_eq!(a.scope().clone(), Triad::try_from((1, 4, 8)).unwrap());
        // Apply the same transformations in reverse to go back to the original
        a.walk(vec![LPR::R, LPR::P, LPR::L]);
        assert_eq!(a.scope().clone(), triad);
    }
}
