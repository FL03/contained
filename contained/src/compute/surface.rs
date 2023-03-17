/*
    Appellation: surface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: a computable surface is a valid universal turing machine
        Generically, a computable surface considers the vertices to be the alphabet
*/
use crate::core::states::State;
use crate::music::{neo::triads::Triad, Notable, Note};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Face<T>(T, State);

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Surface<N: Notable = Note> {
    alphabet: Triad<N>,
    interface: Face<Triad<N>>,
}
