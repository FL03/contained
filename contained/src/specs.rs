/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::Dirac;

pub trait Classifiable<Cls: std::convert::From<i64>> {
    fn class(&self) -> Cls;
}

pub trait Arrays<T>: Clone + ExactSizeIterator + IntoIterator<Item = T> {
    fn as_vec(self) -> Vec<T> {
        Vec::from_iter(self)
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

pub trait Transformable {}

pub trait Transformation<S: Clone> {
    type Output;

    /// [Transformation::dirac] represents a single function capable of transforming one object into another while allowing for errors and null results
    fn dirac(&self) -> &Dirac<&S, Self::Output>;
    /// [Transformation::transform] applies the defined transition to the given objects
    fn transform(&self, args: &S) -> Self::Output {
        self.dirac()(args)
    }
}

pub trait Validity {
    fn is_valid(&self) -> bool;
}
