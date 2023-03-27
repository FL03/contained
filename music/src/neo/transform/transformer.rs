/*
    Appellation: transformer <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{Transform, LPR};
use crate::neo::triads::{Triad, Triadic};
use crate::Note;
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
    pub fn try_to_find(&mut self, note: &Note) -> Option<Vec<LPR>> {
        // Create a queue of (path, triad) tuples
        let mut queue = vec![(Vec::new(), self.scope.lock().unwrap().clone())];
        // While the queue is not empty, pop the last element
        while let Some((path, triad)) = queue.pop() {
            // If the triad contains the note, return the path
            if triad.contains(note) {
                return Some(path);
            }
            for i in LPR::transformations() {
                let mut triad = triad.clone();
                triad.transform(i);
                let mut path = path.clone();
                path.push(i);
                queue.push((path, triad));
            }
        }
        None
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

impl From<Triad> for Transformer {
    fn from(triad: Triad) -> Self {
        Self {
            index: 0,
            iter: Vec::new(),
            scope: Arc::new(Mutex::new(triad)),
        }
    }
}
