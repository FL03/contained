/*
    Appellation: transformer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A transformer is designed to be an asynchronous iterator that applies a series of transformations to a given triad.
*/
use super::{Transform, LPR};
use crate::neo::triads::*;
use futures::Stream;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Poll;

#[derive(Clone, Debug, Default)]
pub struct Transformer {
    index: usize,
    iter: Vec<LPR>,
    scope: Arc<Mutex<Triad>>,
}

impl Transformer {
    pub fn new(iter: impl IntoIterator<Item = LPR>, scope: Arc<Mutex<Triad>>) -> Self {
        Self {
            index: 0,
            iter: Vec::from_iter(iter),
            scope,
        }
    }
}

impl ExactSizeIterator for Transformer {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl Iterator for Transformer {
    type Item = Arc<Mutex<Triad>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.iter.get(self.index) {
            // Increment the index
            self.index += 1;
            // Transform the scope
            self.scope.lock().unwrap().transform(*cur);
            // Return the scope
            Some(self.scope.clone())
        } else {
            None
        }
    }
}

// impl Stream for Transformer {
//     type Item = Arc<Mutex<Triad>>;

//     fn poll_next(
//         self: Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> Poll<Option<Self::Item>> {
//         if self.index == self.iter.len() {
//             Poll::Ready(None)
//         } else {
//             match Pin::new(&mut self.scope) {
//                 Poll::Ready(_) => {
//                     let when = std::time::Duration::from_millis(1);
//                     self.index += 1;
//                     Poll::Ready(Some(()))
//                 }
//                 Poll::Pending => Poll::Pending,
//             }
//         }
//     }
// }

impl From<Triad> for Transformer {
    fn from(triad: Triad) -> Self {
        Self {
            index: 0,
            iter: Vec::new(),
            scope: Arc::new(Mutex::new(triad)),
        }
    }
}
