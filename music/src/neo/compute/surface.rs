/*
    Appellation: surface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: a computable surface is a valid universal turing machine
        Generically, a computable surface considers the vertices to be the alphabet
*/
use crate::neo::triads::Triad;
use crate::{Notable, Note};
use contained_core::states::{State, States};

pub struct Face<T>(T, State);

pub struct Surface<N: Notable> {
    alphabet: Triad<N>,
    faces: (Face<N>,),
}
