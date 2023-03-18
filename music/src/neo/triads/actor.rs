/*
    Appellation: actor <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::Triad;
use crate::neo::LPR;
use crate::{Notable, Note};
use contained_core::states::{State, Stateful};
use contained_core::{turing::tapes::Tape, Scope};
use futures::Stream;
use scsys::prelude::Timestamp;
use serde::{Deserialize, Serialize};
use std::{
    pin::Pin,
    task::Poll,
    time::{Duration, Instant},
};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Actor<N: Notable = Note> {
    index: usize,
    state: State,
    tape: Tape<N>,
    triad: Triad<N>,
    ts: i64,
}

impl<N: Notable> Actor<N> {
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

impl<N: Notable> Iterator for Actor<N> {
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

impl<N: Notable> ExactSizeIterator for Actor<N> {
    fn len(&self) -> usize {
        self.tape.len()
    }
}

impl<N: Notable> Scope<N> for Actor<N> {
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

impl<N: Notable> Stateful<State> for Actor<N> {
    fn state(&self) -> State {
        self.state
    }
    fn update_state(&mut self, state: State) {
        self.state = state;
    }
}

// impl<N: Notable + Unpin> Stream for Instance<N> {
//     type Item = N;

//     fn poll_next(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Option<Self::Item>> {
//         if self.index == 0 {
//             // No more delays
//             return Poll::Ready(None);
//         }

//         match Pin::new(&mut self.triad).poll(cx) {
//             Poll::Ready(_) => {
//                 let when = Instant::now() + Duration::from_millis(10);
//                 self.index -= 1;
//                 Poll::Ready(Some(self.scope().clone()))
//             }
//             Poll::Pending => Poll::Pending,
//         }
//     }
// }

impl<N: Notable> From<Triad<N>> for Actor<N> {
    fn from(triad: Triad<N>) -> Self {
        Self::new(0, Default::default(), Default::default(), triad)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        neo::triads::{Triadic, Triads},
        Note,
    };

    #[test]
    fn test_actor() {
        let triad = Triad::<Note>::new(0.into(), Triads::Major);
        let mut actor = Actor::new(0, State::Valid, Tape::new(triad.clone()), triad.clone());

        actor.shift((-1).into(), triad.third());
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
}
