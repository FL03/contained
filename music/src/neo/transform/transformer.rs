/*
    Appellation: transformer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::LPR;
use crate::{neo::triads::Triad, Notable, Note};
use scsys::prelude::Timestamp;

#[derive(Clone, Debug, Default)]
pub struct Transformer<N: Notable = Note> {
    index: usize,
    iter: Vec<LPR>,
    scope: Triad<N>,
    ts: i64,
}

impl<N: Notable> Transformer<N> {
    pub fn new(iter: impl IntoIterator<Item = LPR>, scope: Triad<N>) -> Self {
        Self {
            index: 0,
            iter: Vec::from_iter(iter),
            scope,
            ts: Timestamp::default().into(),
        }
    }
}

// impl<N: Notable> Iterator Transformer<N> {
//     type Item = Triad<N>;

//     fn next(&mut self) -> Option<Self::Item> {

//         self.ts = Timestamp::default().into();
//         if let Some(cur) = self.iter.next() {
//             Some((self.scope * cur))
//         } else {
//             None
//         }
//     }
// }
