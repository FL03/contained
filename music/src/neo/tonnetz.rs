/*
    Appellation: tonnetz <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        A tonnetz can be any set of connected, non-repeating triads. The tonnetz is essentially a topological computer created by gluing together several triadic machines together.
*/
use super::{triads::Triad, LPR};
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
    use crate::neo::triads::{Triad, Triads};
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
