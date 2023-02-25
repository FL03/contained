/*
    Appellation: actors <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{actor::*, primitives::*, states::*};

pub(crate) mod actor;
pub(crate) mod primitives;
pub(crate) mod states;

pub mod turing;

use serde::{Deserialize, Serialize};

pub fn ordered_sets<T: Clone>(args: Vec<T>, size: usize) -> Vec<Vec<T>> {
    let mut res = Vec::<Vec<T>>::new();
    for i in 0..args.len() {
        let tmp = (1..size)
            .map(|z: usize| (i + z) % size)
            .collect::<Vec<usize>>();
        for j in 0..tmp.len() {
            let mut subset = vec![args[i].clone()];
            subset.append(
                &mut (0..tmp.len())
                    .map(|k: usize| args[tmp[(j + k) % tmp.len()]].clone())
                    .collect(),
            );
            res.push(subset.clone());
        }
    }
    res
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

/// [Appellation] is a novel naming schematic based on a basis from linear-algebra
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Appellation<I, J, K>(I, J, K);

impl<I, J, K> Appellation<I, J, K> {
    pub fn new(a: I, b: J, c: K) -> Self {
        Self(a, b, c)
    }
    pub fn primary(&self) -> &J {
        &self.1
    }
    pub fn root(&self) -> &I {
        &self.0
    }
    pub fn secondary(&self) -> &K {
        &self.2
    }
}
