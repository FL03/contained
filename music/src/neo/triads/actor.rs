/*
    Appellation: actor <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::Triad;
use crate::{neo::LPR, Note};
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
pub struct Actor {
    index: usize,
    state: State,
    tape: Tape<Note>,
    triad: Triad,
    ts: i64,
}

impl Actor {
    pub fn new(index: usize, state: State, tape: Tape<Note>, triad: Triad) -> Self {
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

impl Iterator for Actor {
    type Item = Note;

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

impl ExactSizeIterator for Actor {
    fn len(&self) -> usize {
        self.tape.len()
    }
}

impl Scope<Note> for Actor {
    fn insert(&mut self, elem: Note) {
        self.tape.insert(self.index, elem);
    }

    fn index(&self) -> usize {
        self.index
    }

    fn set_symbol(&mut self, elem: Note) {
        self.tape.set(self.index(), elem);
    }

    fn tape(&self) -> &Tape<Note> {
        &self.tape
    }

    fn set_index(&mut self, pos: usize) {
        self.index = pos;
    }
}

impl Stateful<State> for Actor {
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

impl From<Triad> for Actor {
    fn from(triad: Triad) -> Self {
        Self::new(0, Default::default(), Default::default(), triad)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::neo::triads::{Triadic, Triads};

    #[test]
    fn test_actor() {
        let triad = Triad::new(0.into(), Triads::Major);
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
