/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

/// [ArrayLike] describes the basic behaviors of array-like structures
pub trait ArrayLike<T>: Clone + IntoIterator<Item = T, IntoIter = std::vec::IntoIter<T>> {
    /// [ArrayLike::content]
    fn content(&self) -> &Vec<T>;
    /// [ArrayLike::mut_content]
    fn mut_content(&mut self) -> &mut Vec<T>;
    /// [ArrayLike::append] describes a method which takes another similar array and adds the values to the end of the current array
    fn append(&mut self, elem: &mut Vec<T>) {
        self.mut_content().append(elem);
    }
    /// [ArrayLike::extend] describes a method for extending the array with values from another, similar array
    fn extend(&mut self, elem: impl IntoIterator<Item = T>) {
        self.mut_content().extend(Vec::from_iter(elem));
    }
    /// [ArrayLike::insert] describes a method for inserting a new element at a specific position
    fn insert(&mut self, index: usize, elem: T) {
        self.mut_content().insert(index, elem)
    }
    /// [ArrayLike::is_empty] determines if the array is empty or not
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// [ArrayLike::len] determine the length of the content
    fn len(&self) -> usize {
        self.clone().into_iter().count()
    }
}

/// [Extend]
pub trait Extend<A> {
    type Output;

    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) -> Self::Output;
}

/// [With] describes a simple means of concating several objects together
pub trait With<T> {
    /// [With::Output] must be a superposition of self and T
    type Output;

    /// [With::with] accepts an owned instance of the given type and returns a [With::Output] instance
    fn with(&self, other: &T) -> Self::Output;
}
