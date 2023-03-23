/*
    Appellation: exec <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::Execute;
use crate::turing::Program;
use crate::Symbolic;

pub struct Executor<S: Symbolic> {
    program: Program<S>,
}
