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

    pub fn origin(mut self, triad: T) -> Self {
        self.queue.push((Vec::new(), triad));
        self
    }

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
}
