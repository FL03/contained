/*
    Appellation: transformer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::LPR;
use crate::neo::triads::{Triad, Triadic};
use std::sync::{Arc, Mutex};

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
