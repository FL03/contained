/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A triad is a set of three notes, called chord factors: root, third, and fifth
        Generaically, triad's share two of its notes with three of its inversions.

        We express a triad as a ordered three tuple <a, b, c> where a, b, c are integers modulus of 12 and:
            a != b
            a != c
            b != c
*/
use crate::neo::cmp::{Note, PitchClass};
use crate::turing::Symbolic;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Triad(Note, Note, Note);

impl Triad {
    pub fn new(root: Note, third: Note, fifth: Note) -> Self {
        Self(root, third, fifth)
    }
    pub fn is_valid(&self) -> bool {
        if self.0 != self.1 && self.0 != self.2 && self.1 != self.2 {
            return true;
        }
        false
    }
    pub fn fifth(&self) -> &Note {
        &self.2
    }
    pub fn root(&self) -> &Note {
        &self.0
    }
    pub fn third(&self) -> &Note {
        &self.1
    }
}

impl std::fmt::Display for Triad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

impl TryFrom<(&str, &str, &str)> for Triad {
    type Error = std::string::ParseError;

    fn try_from(d: (&str, &str, &str)) -> Result<Triad, Self::Error> {
        let (root, third, fifth) = (Note::try_from(d.0)?, Note::try_from(d.1)?, Note::try_from(d.2)?);
        Ok(Triad::new(root, third, fifth))
    }
}

impl From<(Note, Note, Note)> for Triad {
    fn from(d: (Note, Note, Note)) -> Triad {
        Triad(d.0, d.1, d.2)
    }
}

impl From<Triad> for (Note, Note, Note) {
    fn from(d: Triad) -> (Note, Note, Note) {
        (d.0, d.1, d.2)
    }
}
