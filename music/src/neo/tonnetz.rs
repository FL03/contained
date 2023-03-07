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

        While one instance can only occupy a single configuration at a time, the tonnetz can break down the program into executable pieces
        injecting in LPR transformations to gaurentee execution. This process can be scaled as having multiple persistent instances allows
        the system to offload certain workloads without disrupting the overall experience.

        Each subnet is orchestrated relative to the originator; the originator is considered to be the triadic structure being actively engaged with by the user.
*/
use super::{Triad, LPR};
use crate::{Notable, Note};
use std::sync::Arc;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tonnetz<N: Notable = Note> {
    scope: Arc<Triad<N>>,
}

impl<N: Notable> Tonnetz<N> {
    pub fn new(scope: Arc<Triad<N>>) -> Self {
        Self { scope }
    }
    pub fn cycle(&mut self, iter: impl IntoIterator<Item = LPR>) {
        for i in Vec::from_iter(iter).iter().cycle() {
            self.transform(*i)
        }
    }
    /// Returns an owned instance of the active [Triad]
    pub fn scope(&self) -> &Triad<N> {
        self.scope.as_ref()
    }
    pub fn transform(&mut self, shift: LPR) {
        self.scope = Arc::new(self.scope().clone() * shift);
    }

    pub fn walk(&mut self, iter: impl IntoIterator<Item = LPR>) {
        for s in iter {
            self.transform(s)
        }
    }
}

impl<N: Notable> std::fmt::Display for Tonnetz<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.scope())
    }
}

impl<N: Notable> From<Triad<N>> for Tonnetz<N> {
    fn from(triad: Triad<N>) -> Tonnetz<N> {
        Tonnetz::<N>::new(Arc::new(triad))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neo::{Triad, Triads};
    use crate::Note;

    #[test]
    fn test_tonnetz() {
        let triad = Triad::<Note>::new(0.into(), Triads::Major);

        let mut a = Tonnetz::from(triad.clone());
        // Apply three consecutive transformations to the scope
        a.walk(vec![LPR::L, LPR::P, LPR::R]);
        assert_eq!(a.scope().clone(), Triad::try_from((1, 4, 8)).unwrap());
        // Apply the same transformations in reverse to go back to the original
        a.walk(vec![LPR::R, LPR::P, LPR::L]);
        assert_eq!(a.scope().clone(), triad);
    }
}
