/*
    Appellation: triads <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A triad is a set of three notes, called chord factors: root, third, and fifth
        Generaically, triad's share two of its notes with three of its inversions.

        For our purposes, a triad is said to be a three-tuple (a, b, c) where the intervals [a, b] and [b, c] are both thirds.
*/

pub use self::{class::*, misc::*, triad::*};

mod class;
mod misc;
mod triad;

pub trait Update {
    fn update(&mut self);
}

pub trait FromTriad {
    fn from_triad(triad: Triad) -> Self;
}

impl<T> FromTriad for T
where
    T: From<Triad>,
{
    fn from_triad(triad: Triad) -> Self {
        Self::from(triad)
    }
}

pub trait IntoTriad {
    fn into_triad(self) -> Triad;
}

impl<T> IntoTriad for T
where
    T: Into<Triad>,
{
    fn into_triad(self) -> Triad {
        self.into()
    }
}

pub trait TryIntoTriad {
    type Error;

    fn try_into_triad(self) -> Result<Triad, Self::Error>;
}

impl<T> TryIntoTriad for T
where
    T: TryInto<Triad>,
    T::Error: std::error::Error,
{
    type Error = T::Error;

    fn try_into_triad(self) -> Result<Triad, Self::Error> {
        self.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neo::LPR;

    #[test]
    fn test_triad() {
        let a = Triad::new(0.into(), Triads::Major);
        assert_eq!(a.clone().as_ref(), &[0.into(), 4.into(), 7.into()]);
        let tmp: (i64, i64, i64) = a.into();
        assert_eq!(tmp, (0, 4, 7));
        let b = Triad::try_from((11, 4, 7));
        assert!(b.is_ok());
        assert_ne!(a, b.unwrap())
    }

    #[test]
    fn test_walking() {
        let expected = Triad::try_from([1, 4, 8]).unwrap();
        let triad = Triad::new(0.into(), Triads::Major);

        let mut a = triad;
        let mut b = triad;
        // Apply three consecutive transformations to the scope
        a.walk(vec![LPR::L, LPR::P, LPR::R]);
        assert_eq!(a.clone(), expected);
        // Apply the same transformations in reverse to go back to the original
        a.walk(vec![LPR::R, LPR::P, LPR::L]);
        b.yoyo(vec![LPR::L, LPR::P, LPR::R]);
        assert_eq!(a, b);
    }
}
