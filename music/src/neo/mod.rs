/*
    Appellation: neo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Neo
//!
//! This module is dedicated to the neo-Riemannian theory of music and its computational implications.
//!
pub use self::{pathfinder::*, transform::*};

mod pathfinder;
mod transform;

pub mod tonnetz;
pub mod triads;

#[cfg(test)]
mod tests {
    use super::*;
    use triads::*;

    #[test]
    fn test_pathfinder() {
        let triad = Triad::new(0.into(), Triads::Major);
        for i in [1, 3, 11] {
            let mut pathfinder = PathFinder::new(i.into()).set_origin(triad.clone());
            assert!(pathfinder.find().is_some());
        }
    }
}
