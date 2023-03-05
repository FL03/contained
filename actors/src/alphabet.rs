/*
    Appellation: alphabet <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::Symbolic;

pub trait Alphabet<S: Symbolic>: Clone + IntoIterator<Item = S> {
    /// [Alphabet::alphabet]
    fn alphabet(self) -> Vec<S> {
        Vec::from_iter(self)
    }
    /// [Alphabet::default_symbol]
    fn default_symbol(&self) -> S {
        match self.clone().alphabet().first() {
            Some(v) => v.clone(),
            None => Default::default(),
        }
    }
}

impl<S: Symbolic> Alphabet<S> for Vec<S> {}
