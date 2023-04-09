/*
    Appellation: triads <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A triad is a set of three notes, called chord factors: root, third, and fifth
        Generaically, triad's share two of its notes with three of its inversions.

        For our purposes, a triad is said to be a three-tuple (a, b, c) where the intervals [a, b] and [b, c] are both thirds.
*/

pub use self::{class::*, surface::*, triad::*};

mod class;
mod surface;
mod triad;

pub trait IntoTriad {
    fn into_triad(self) -> Triad;
}

pub trait TryIntoTriad {
    type Error;

    fn try_into_triad(self) -> Result<Triad, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neo::LPR;

    #[test]
    fn test_triad() {
        let a = Triad::new(0.into(), TriadClass::Major);
        assert_eq!(a.clone().as_ref(), &[0.into(), 4.into(), 7.into()]);
        let tmp: (i64, i64, i64) = a.clone().into();
        assert_eq!(tmp, (0, 4, 7));
        let b = Triad::try_from((11, 4, 7));
        assert!(b.is_ok());
        assert_ne!(a, b.unwrap())
    }

    #[test]
    fn test_walking() {
        let expected = Triad::try_from([1, 4, 8]).unwrap();
        let triad = Triad::new(0.into(), TriadClass::Major);

        let mut a = triad.clone();
        let mut b = triad.clone();
        // Apply three consecutive transformations to the scope
        a.walk(vec![LPR::L, LPR::P, LPR::R]);
        assert_eq!(a.clone(), expected);
        // Apply the same transformations in reverse to go back to the original
        a.walk(vec![LPR::R, LPR::P, LPR::L]);
        b.yoyo(vec![LPR::L, LPR::P, LPR::R]);
        assert_eq!(a, b);
    }
}
