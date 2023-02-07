/*
    Appellation: Contained <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::states::*;

pub(crate) mod states;

pub mod neo;
pub mod turing;

use serde::{Deserialize, Serialize};

/// Dirac is a generic [Fn] which transforms one object into another
pub type Dirac<S, T> = dyn Fn(S) -> T;
/// Type alias for a [Result]
pub type Resultant<T = (), E = String> = Result<T, E>;

/// [Appellation] is a unique naming schematic which relates two optionally parameters with a 'root'
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub struct Appellation<T: Clone + Default = i64>(T, Option<T>, Option<T>);

impl<T: Clone + Default> Appellation<T> {
    pub fn new(a: T, b: Option<T>, c: Option<T>) -> Self {
        Self(a, b, c)
    }
}
