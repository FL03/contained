/*
    Appellation: neo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        This module is dedicated to the proposed harmonic computational fabric
*/
pub use self::{pathfinder::*, transform::*};

mod pathfinder;
mod transform;

pub mod triads;
pub mod tonnetz;

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