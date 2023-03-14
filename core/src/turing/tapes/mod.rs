/*
    Appellation: tapes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: The tape structure modifies traditional vectors, restricing the ability to remove entries from the tape.
*/
pub use self::{builder::*, tape::*};

pub(crate) mod builder;
pub(crate) mod tape;

use crate::Symbolic;

pub trait Taped<S: Symbolic>:
    Clone + IntoIterator<Item = S, IntoIter = std::vec::IntoIter<S>>
{
    fn tape(self) -> Vec<S> {
        Vec::from_iter(self.clone())
    }
}
