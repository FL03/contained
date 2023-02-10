/*
    Appellation: Contained <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::states::*;

pub(crate) mod states;

pub mod neo;
pub mod turing;

use serde::{Deserialize, Serialize};

/// Dirac is a generic [Fn] which transforms one object into another
pub type Dirac<S, T> = dyn Fn(S) -> T;
/// Type alias for a [Result]
pub type Resultant<T = (), E = String> = Result<T, E>;

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

/// [Appellation] is a novel naming schematic which provides up to three variations of the same data
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub struct Appellation<T: Clone + Default = i64>(T, Option<T>, Option<T>);

impl<T: Clone + Default> Appellation<T> {
    pub fn new(a: T, b: Option<T>, c: Option<T>) -> Self {
        Self(a, b, c)
    }
}
