/*
    Appellation: cmp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A collection of musical objects abstracted for computational purposes
*/
pub use self::{chord::*, clef::*, epoch::*, intervals::*, notes::*, pitch::*};

pub(crate) mod chord;
pub(crate) mod clef;
pub(crate) mod epoch;
pub(crate) mod intervals;
pub(crate) mod notes;
pub(crate) mod pitch;

pub trait Gradient {
    fn pitch(&self) -> i64;
    /// Simple way to detect if the pitch is natural or not
    fn is_natural(&self) -> bool {
        NaturalNote::try_from(self.pitch()).is_ok()
    }
}

impl Gradient for i64 {
    fn pitch(&self) -> i64 {
        // Adding twelve to the number accounts for negative modulo
        // For example, if self is -1 than adding 12 gives us a result of 11.
        (((self % 12) + 12) % 12).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::Gradient;

    #[test]
    fn test_gradient() {
        let b = -13;
        assert_eq!(144_i64.pitch(), 0);
        assert_eq!(b.pitch(), 11)
    }
}
