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

impl<N: Notable> Iterator for Transformer<N> {
    type Item = Triad<N>;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.index;
        self.index += 1;
        self.ts = Timestamp::default().into();
        if let Some(cur) = self.iter.get(i) {
            self.scope.transform(*cur);
            Some(self.scope.clone())
        } else {
            None
        }
    }
}

// impl<N: Notable + Unpin> Stream for Transformer<N> {
//     type Item = Triad<N>;

//     fn poll_next(
//         self: Pin<&mut Self>,
//         cx: &mut task::Context<'_>,
//     ) -> Poll<Option<Self::Item>> {
//         if self.index == self.iter.len() {
//             // No more delays
//             return Poll::Ready(None);
//         }

//         match Pin::new(&mut self.scope).poll(cx) {
//             Poll::Ready(_) => {
//                 let when = Instant::now() + Duration::new(1, 0);
//                 let i = self.index;
//                 self.index += 1;
//                 if let Some(cur) = self.iter.get(i) {
//                     self.scope.transform(*cur);
//                     Poll::Ready(Some(self.scope.as_ref().clone()))
//                 } else {
//                     Poll::Ready(None)
//                 }
//             }
//             Poll::Pending => Poll::Pending,
//         }
//     }
// }