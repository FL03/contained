/*
    Appellation: triads <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A triad is a set of three notes, called chord factors: root, third, and fifth
        Generaically, triad's share two of its notes with three of its inversions.

        For our purposes, a triad is said to be a three-tuple (a, b, c) where the intervals [a, b] and [b, c] are both thirds.
*/
pub use self::{class::*, triad::*};

pub(crate) mod class;
pub mod tonic;
pub(crate) mod triad;

use crate::Notable;
use contained_core::states::{State, Stateful};
use contained_core::{turing::tapes::Tape, Scope};
use scsys::prelude::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Instance<N: Notable> {
    index: usize,
    state: State,
    tape: Tape<N>,
    triad: Triad<N>,
    ts: i64,
}

impl<N: Notable> Instance<N> {
    pub fn new(index: usize, state: State, tape: Tape<N>, triad: Triad<N>) -> Self {
        let ts = Timestamp::default().into();
        Self {
            index,
            state,
            tape,
            triad,
            ts,
        }
    }
}

impl<N: Notable> Iterator for Instance<N> {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.index;
        self.index += 1;
        self.ts = Timestamp::default().into();
        if let Some(cur) = self.tape.get(i) {
            Some(cur.clone())
        } else {
            None
        }
    }
}

impl<N: Notable> Scope<N> for Instance<N> {
    fn insert(&mut self, elem: N) {
        self.tape.insert(self.index, elem);
    }

    fn index(&self) -> usize {
        self.index
    }

    fn set_symbol(&mut self, elem: N) {
        self.tape.set(self.index(), elem);
    }

    fn tape(&self) -> &Tape<N> {
        &self.tape
    }

    fn set_index(&mut self, pos: usize) {
        self.index = pos;
    }
}

impl<N: Notable> Stateful<State> for Instance<N> {
    fn state(&self) -> State {
        self.state
    }
    fn update_state(&mut self, state: State) {
        self.state = state;
    }
}

impl<N: Notable> From<Triad<N>> for Instance<N> {
    fn from(triad: Triad<N>) -> Self {
        Self::new(0, Default::default(), Default::default(), triad)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{neo::LPR, Note};

    #[test]
    fn test_driver() {
        let triad = Triad::<Note>::new(0.into(), Triads::Major);
        let mut actor = Instance::new(0, State::Valid, Tape::new(triad.clone()), triad.clone());

        actor.shift(0.into(), triad.third());
        assert_eq!(
            actor.tape(),
            &Tape::new([4.into(), 0.into(), 4.into(), 7.into()])
        );
        for _ in 0..actor.tape().len() {
            actor.shift(1.into(), triad.fifth());
        }
        assert_eq!(
            actor.tape(),
            &Tape::new([4.into(), 0.into(), 4.into(), 7.into(), 7.into()])
        );
    }

    #[test]
    fn test_triad() {
        let a = Triad::<Note>::new(0.into(), Triads::Major);
        let tmp: (i64, i64, i64) = a.clone().into();
        assert_eq!(tmp, (0, 4, 7));
        let b = Triad::try_from((11, 4, 7));
        assert!(b.is_ok());
        assert_ne!(a, b.unwrap())
    }

    #[test]
    fn test_walking() {
        let triad = Triad::<Note>::new(0.into(), Triads::Major);

        let mut a = triad.clone();
        // Apply three consecutive transformations to the scope
        a.walk(vec![LPR::L, LPR::P, LPR::R]);
        assert_eq!(a.clone(), Triad::try_from((1, 4, 8)).unwrap());
        // Apply the same transformations in reverse to go back to the original
        a.walk(vec![LPR::R, LPR::P, LPR::L]);
        assert_eq!(a.clone(), triad);
    }
}
