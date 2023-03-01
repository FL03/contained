/*
    Appellation: actors <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{actor::*, primitives::*, states::*, utils::*};

pub(crate) mod actor;
pub(crate) mod primitives;
pub(crate) mod states;
pub(crate) mod utils;

pub mod turing;

use serde::{Deserialize, Serialize};

/// Simple trait for compatible symbols
pub trait Symbolic:
    Clone
    + Default
    + Eq
    + Ord
    + PartialEq
    + PartialOrd
    + std::fmt::Debug
    + std::fmt::Display
    + serde::Serialize
{
}

impl Symbolic for char {}

impl Symbolic for &str {}

impl Symbolic for String {}

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
