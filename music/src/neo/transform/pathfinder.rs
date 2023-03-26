/*
    Appellation: pathfinder <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::LPR;
use crate::neo::triads::*;
use crate::Note;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd)]
pub struct PathFinder<T: Triadic = Triad> {
    queue: Vec<(Vec<LPR>, T)>,
    target: Note,
}

impl<T: Triadic> PathFinder<T> {
    pub fn new(target: Note) -> Self {
        Self {
            queue: Vec::new(),
            target,
        }
    }
    /// Finds the shortest path to the target
    pub fn find(&mut self) -> Option<Vec<LPR>> {
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
    /// Resets the pathfinder
    pub fn reset(&mut self) {
        self.queue.clear();
    }
    /// Sets the origin of the pathfinder
    pub fn set_origin(mut self, triad: T) -> Self {
        self.queue.push((Vec::new(), triad));
        self
    }
    /// Sets the target of the pathfinder
    pub fn set_target(&mut self, target: Note) {
        self.target = target;
    }
}
