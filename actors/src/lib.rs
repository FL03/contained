/*
    Appellation: actors <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{primitives::*, scope::*, utils::*};

pub(crate) mod primitives;
pub(crate) mod scope;
pub(crate) mod utils;

pub mod states;
pub mod turing;

pub trait Alphabet<S: Symbolic>: Clone + std::iter::IntoIterator<Item = S> {
    fn alphabet(&self) -> Vec<S> {
        Vec::from_iter(self.clone())
    }
    fn default_symbol(&self) -> &S;
}

impl<S: Symbolic> Alphabet<S> for Vec<S> {
    fn default_symbol(&self) -> &S {
        &self[0]
    }
}

pub trait Extend<A> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) -> Result<(), String>;
}

/// Simple trait for compatible symbols
pub trait Symbolic: Clone + Default + PartialEq + std::fmt::Debug + std::fmt::Display {}

impl Symbolic for char {}

impl Symbolic for &str {}

impl Symbolic for String {}

/// [With] describes a simple means of concating several objects together
pub trait With<T> {
    /// [With::Output] must be a superposition of self and T
    type Output;

    /// [With::with] accepts an owned instance of the given type and returns a [With::Output] instance
    fn with(&self, other: &T) -> Self::Output;
}
