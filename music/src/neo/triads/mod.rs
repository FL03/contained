/*
    Appellation: triads <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A triad is a set of three notes, called chord factors: root, third, and fifth
        Generaically, triad's share two of its notes with three of its inversions.

        For our purposes, a triad is said to be a three-tuple (a, b, c) where the intervals [a, b] and [b, c] are both thirds.
*/

pub use self::{class::*, triad::*};

pub(crate) mod class;
pub(crate) mod triad;

pub mod actor;
pub mod tonic;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{neo::LPR, Note};

    #[test]
    fn test_triad() {
        let a = Triad::<Note>::new(0.into(), Triads::Major);
        let tmp: (i64, i64, i64) = a.clone().into();
        assert_eq!(tmp, (0, 4, 7));
        let b = Triad::try_from((11, 4, 7));
        assert!(b.is_ok());
        assert_ne!(a, b.unwrap())
    }

    #[test]
    fn test_walking() {
        let triad = Triad::<Note>::new(0.into(), Triads::Major);

        let mut a = triad.clone();
        // Apply three consecutive transformations to the scope
        a.walk(vec![LPR::L, LPR::P, LPR::R]);
        assert_eq!(a.clone(), Triad::try_from((1, 4, 8)).unwrap());
        // Apply the same transformations in reverse to go back to the original
        a.walk(vec![LPR::R, LPR::P, LPR::L]);
        assert_eq!(a.clone(), triad);
    }
}
