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
    fn class(&self) -> PitchClass {
        PitchClass::from(self.pitch())
    }
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

/// [ArrayLike] describes the basic behaviors of array-like structures
pub trait ArrayLike {
    type Data;

    /// [ArrayLike::content]
    fn content(&self) -> &Vec<Self::Data>;
    /// [ArrayLike::mut_content]
    fn mut_content(&mut self) -> &mut Vec<Self::Data>;
    /// [ArrayLike::append] describes a method which takes another similar array and adds the values to the end of the current array
    fn append(&mut self, elem: &mut Vec<Self::Data>) {
        self.mut_content().append(elem);
    }
    /// [ArrayLike::extend] describes a method for extending the array with values from another, similar array
    fn extend(&mut self, elem: impl IntoIterator<Item = Self::Data>) {
        self.mut_content().extend(Vec::from_iter(elem));
    }
    /// [ArrayLike::insert] describes a method for inserting a new element at a specific position
    fn insert(&mut self, index: usize, elem: Self::Data) {
        self.mut_content().insert(index, elem)
    }
    /// [ArrayLike::is_empty] determines if the array is empty or not
    fn is_empty(&self) -> bool {
        self.content().is_empty()
    }
    /// [ArrayLike::len] determine the length of the content
    fn len(&self) -> usize {
        self.content().len()
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
