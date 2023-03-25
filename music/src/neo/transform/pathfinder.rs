/*
    Appellation: pathfinder <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{Transform, LPR};
use crate::neo::triads::*;
use crate::Note;

pub struct PathFinder {
    queue: Vec<(Vec<LPR>, Triad)>,
    target: Note,
}

impl PathFinder {
    pub fn new(target: Note) -> Self {
        Self {
            queue: Vec::new(),
            target,
        }
    }

    pub fn find(&mut self, triad: Triad) -> Option<Vec<LPR>> {
        self.queue.push((Vec::new(), triad));
        while let Some((path, triad)) = self.queue.pop() {
            if triad.contains(&self.target) {
                return Some(path);
            }
            for i in LPR::transformations() {
                let mut triad = triad.clone();
                triad.transform(i);
                let mut path = path.clone();
                path.push(i);
                if triad.contains(&self.target) {
                    return Some(path);
                }
                self.queue.push((path, triad));
            }
        }
        None
    }
}
