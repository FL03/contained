/*
    Appellation: surface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: a computable surface is a valid universal turing machine
        Generically, a computable surface considers the vertices to be the alphabet
*/
use crate::neo::triads::Triad;
use crate::{Notable, Note};
use contained_core::states::State;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Face<T>(T, State);

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Surface<N: Notable = Note> {
    alphabet: Triad<N>,
    faces: (Face<N>,),
}
