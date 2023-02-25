/*
    Appellation: actor <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::Symbolic;

pub trait Actor<S: Symbolic> {
    fn seed(&self) -> &S;
}
