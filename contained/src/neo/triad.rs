/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A triad is a set of three notes, called chord factors: root, third, and fifth

        Computationally, the triad is a three-tuple of 


*/
use crate::neo::{Note, Pitch};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Root<P: Pitch>(Note<P>);

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Third<P: Pitch>(Note<P>);

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Fifth<P: Pitch>(Note<P>);

///
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Triad<I: Pitch, J: Pitch, K: Pitch>(Root<I>, Third<J>, Fifth<K>);

impl<I: Pitch, J: Pitch, K: Pitch> Triad<I, J, K> {
    pub fn new(root: Root<I>, third: Third<J>, fifth: Fifth<K>) -> Self {
        Self(root, third, fifth)
    }
}
