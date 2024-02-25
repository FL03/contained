/*
    Appellation: transformer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A transformer is designed to be an asynchronous iterator that applies a series of transformations to a given triad.
*/
use super::{Transform, LPR};
use crate::neo::triads::*;
use futures::Stream;
use itertools::Itertools;
use std::future::Future;
use std::task::{self, Poll};

#[derive(Clone, Debug, Default)]
pub struct Transformer {
    index: usize,
    iter: Vec<LPR>,
    scope: Triad,
}

impl Transformer {
    pub fn new(scope: Triad) -> Self {
        Self {
            index: 0,
            iter: Vec::new(),
            scope,
        }
    }
    pub fn push(&mut self, lpr: LPR) {
        self.iter.push(lpr);
    }
    pub fn with(mut self, iter: impl IntoIterator<Item = LPR>) -> Self {
        self.iter = iter.into_iter().collect_vec();
        self
    }
}

impl Extend<LPR> for Transformer {
    fn extend<T: IntoIterator<Item = LPR>>(&mut self, iter: T) {
        self.iter.extend(iter);
    }
}

impl ExactSizeIterator for Transformer {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl Iterator for Transformer {
    type Item = Triad;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.iter.get(self.index) {
            // Increment the index
            self.index += 1;
            // Transform the scope
            self.scope.transform(*cur);
            // Return the scope
            Some(self.scope.clone())
        } else {
            None
        }
    }
}

impl Future for Transformer {
    type Output = Triad;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        cx.waker().wake_by_ref();
        if let None = self.next() {
            return Poll::Ready(self.scope.clone());
        }
        return Poll::Pending;
    }
}

impl Stream for Transformer {
    type Item = Triad;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        cx.waker().wake_by_ref();
        if let Some(cur) = self.next() {
            Poll::Ready(Some(cur))
        } else {
            Poll::Ready(None)
        }
    }
}

impl Unpin for Transformer {}

impl std::ops::Index<usize> for Transformer {
    type Output = LPR;

    fn index(&self, index: usize) -> &Self::Output {
        &self.iter[index]
    }
}

impl std::ops::IndexMut<usize> for Transformer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.iter[index]
    }
}

impl From<Triad> for Transformer {
    fn from(scope: Triad) -> Self {
        Self {
            index: 0,
            iter: Vec::new(),
            scope,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use strum::IntoEnumIterator;
    use LPR::*;

    lazy_static! {
        static ref _EXPECTED: (Triad, Vec<Triad>) = {
            let mut triad = Triad::default();
            let prev = triad.walk_across([L, P, R]);
            (triad, prev)
        };
    }

    #[test]
    fn test_transformer() {
        let triad = Triad::default();
        let transformer = Transformer::new(triad).with(LPR::iter());
        let (expected, walked) = _EXPECTED.clone();
        for (i, t) in transformer.enumerate() {
            if i >= walked.len() {
                assert_eq!(t, expected);
            } else {
                assert_eq!(t, walked[i]);
            }
        }
    }

    #[tokio::test]
    async fn test_transformer_future() {
        let triad = Triad::default();
        let (expected, _walked) = _EXPECTED.clone();
        let transformer = Transformer::new(triad).with(LPR::iter());
        assert_eq!(transformer.await, expected);
    }

    #[tokio::test]
    async fn test_stream_transformer() {
        use futures::{stream, StreamExt};
        let triad = Triad::default();
        let (expected, walked) = _EXPECTED.clone();
        let transformer = Transformer::new(triad).with(LPR::iter());
        let s = stream::iter(transformer);
        let res = s.collect::<Vec<_>>().await;
        for (i, triad) in res.clone().into_iter().enumerate() {
            if i >= walked.len() {
                assert_eq!(triad, expected);
            } else {
                assert_eq!(triad, walked[i]);
            }
        }
    }
}
