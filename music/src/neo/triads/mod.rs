/*
    Appellation: triads <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! Triad
//!
//! def. A triad is a set of three notes, called chord factors
//!
//! # Capabilities
//!
//! Justification for considering triads to be viable topological computing environment is found with the [Wolfram (2, 3) UTM](https://www.wolframscience.com/prizes/tm23).
//! Generally, a universal turing machine is capable of emulating any other turing machine. The (2, 3) UTM is a turing machine that can emulate any other turing machine using only two states and three symbols.
//! Considering a triad to be a three-tuple (a, b, c) where the intervals [a, b] and [b, c] are both thirds, we can see that the triad is a (2, 3) UTM where each side or "state" is consistently allowed to be either
//! invalid or valid.
//!
pub use self::{builder::*, class::*, factors::*, graph::*, triad::*};

mod builder;
mod class;
mod factors;
mod triad;

pub mod graph;

/// [FromTriad] is a simple trait that allows for the explicit conversion of a [Triad] into any type that implements [From<Triad>].
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

/// [IntoTriad] is a simple trait that allows for the explicit conversion of any type that implements [Into<Triad>] into a [Triad].
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

/// [TryIntoTriad] is a trait for explicitly attempting to convert any type into a [Triad].
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
