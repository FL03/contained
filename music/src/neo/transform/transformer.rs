/*
    Appellation: transformer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A transformer is designed to be an asynchronous iterator that applies a series of transformations to a given triad.
*/
use super::{Transform, LPR};
use crate::neo::triads::*;

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
    pub fn with(mut self, iter: Vec<LPR>) -> Self {
        self.iter = iter;
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
