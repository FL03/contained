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
use super::LPR;
use crate::cmp::{
    is_major_third, is_minor_third, is_third, major_third, minor_third, perfect_fifth, Note,
};
use crate::turing::{Configuration, Symbolic, Tape};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

/// [classify_triad] detects if the given triad is augments, diminshed, major, or minor
pub fn classify_triad(triad: &Triad) -> Option<Triads> {
    let (r, t, f) = triad.clone().into();

    if perfect_fifth(r) == f {
        if is_major_third(r, t) {
            return Some(Triads::Major);
        } else {
            return Some(Triads::Minor);
        }
    } else {
        if is_major_third(r, t) && is_major_third(t, f) {
            return Some(Triads::Augmented);
        } else if is_minor_third(r, t) && is_minor_third(t, f) {
            return Some(Triads::Diminshed);
        }
        return None;
    }
}

/// [create_triad] trys to create a triad from the given notes
/// This is accomplished by 'discovering' which order of the notes satisfies the minimum relationships
/// Since we allow for augmented / diminshed triads, the root -> third && third -> fifth are required to be thirds
/// rather than enforcing a 'perfect fifth' relationship between root -> fifth
pub fn create_triad(notes: (Note, Note, Note)) -> Option<Triad> {
    let args = vec![notes.0, notes.1, notes.2];
    for i in 0..args.len() {
        let tmp = [(i + 1) % args.len(), (i + 2) % args.len()];
        for j in 0..tmp.len() {
            let (a, b, c) = (
                args[i].clone(),
                args[tmp[j]].clone(),
                args[tmp[(j + 1) % tmp.len()]].clone(),
            );
            // Creates a triad if the current root -> current third & current third -> current fifth are both thirds
            if is_third(a.clone().into(), b.clone().into())
                && is_third(b.clone().into(), c.clone().into())
            {
                return Some(Triad::new(a, b, c));
            }
        }
    }
    None
}

pub trait Triadic {
    fn fifth(&self) -> Note;
    fn root(&self) -> Note;
    fn third(&self) -> Note;
    fn triad(&self) -> (Note, Note, Note) {
        (self.root(), self.third(), self.fifth())
    }
}

impl Triadic for (Note, Note, Note) {
    fn fifth(&self) -> Note {
        self.2.clone()
    }

    fn root(&self) -> Note {
        self.0.clone()
    }

    fn third(&self) -> Note {
        self.1.clone()
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum Triads {
    Augmented, // If the root -> third is major and if third -> fifth is major
    Diminshed, // If the root -> third is minor and if third -> fifth is minor
    #[default]
    Major, // If the root -> third is major and if third -> fifth is minor
    Minor, // If the root -> third is minor and if third -> fifth is major
}

impl Triads {
    pub fn create(&self, root: Note) -> Triad {
        let pitch: i64 = root.clone().into();
        let (third_maj, third_minor) = (major_third(pitch), minor_third(pitch));
        match self.clone() {
            Triads::Augmented => Triad::new(
                root,
                Note::from(third_maj),
                Note::from(major_third(third_maj)),
            ),
            Triads::Diminshed => Triad::new(
                root,
                Note::from(third_minor),
                Note::from(minor_third(third_minor)),
            ),
            Triads::Major => Triad::new(
                root,
                Note::from(third_maj),
                Note::from(minor_third(third_maj)),
            ),
            Triads::Minor => Triad::new(
                root,
                Note::from(third_minor),
                Note::from(major_third(third_minor)),
            ),
        }
    }
}

impl TryFrom<Triad> for Triads {
    type Error = String;

    fn try_from(data: Triad) -> Result<Triads, Self::Error> {
        let (r, t, f) = data.clone().into();

        if perfect_fifth(r) == f {
            if is_major_third(r, t) {
                return Ok(Triads::Major);
            } else {
                return Ok(Triads::Minor);
            }
        } else {
            if is_major_third(r, t) && is_major_third(t, f) {
                return Ok(Triads::Augmented);
            } else if is_minor_third(r, t) && is_minor_third(t, f) {
                return Ok(Triads::Diminshed);
            }
            return Err("".to_string());
        }
    }
}

/// [Triad] is a set of three [Note], the root, third, and fifth.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Triad(Note, Note, Note);

impl Triad {
    pub fn new(root: Note, third: Note, fifth: Note) -> Self {
        Self(root, third, fifth)
    }
    pub fn build(root: Note, class: Triads) -> Self {
        let pitch: i64 = root.clone().into();
        let (third_maj, third_minor) = (major_third(pitch), minor_third(pitch));
        match class {
            Triads::Augmented => Self::new(
                root,
                Note::from(third_maj),
                Note::from(major_third(third_maj)),
            ),
            Triads::Diminshed => Self::new(
                root,
                Note::from(third_minor),
                Note::from(minor_third(third_minor)),
            ),
            Triads::Major => Self::new(
                root,
                Note::from(third_maj),
                Note::from(minor_third(third_maj)),
            ),
            Triads::Minor => Self::new(
                root,
                Note::from(third_minor),
                Note::from(major_third(third_minor)),
            ),
        }
    }
    /// Create a new [Configuration] with the [Triad] as its alphabet
    pub fn config(&self) -> Configuration<Note> {
        Configuration::norm(Tape::new(self.clone())).unwrap()
    }
    /// A method for establishing the validity of the given notes
    pub fn is_valid(&self) -> bool {
        if classify_triad(&self).is_some() {
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

impl Symbolic for Triad {}

impl std::fmt::Display for Triad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0, self.1, self.2)
    }
}

impl std::ops::Mul<LPR> for Triad {
    type Output = Triad;

    fn mul(self, rhs: LPR) -> Self::Output {
        rhs.transform(&mut self.clone())
    }
}

impl IntoIterator for Triad {
    type Item = Note;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.0, self.1, self.2].into_iter()
    }
}

impl TryFrom<(i64, i64, i64)> for Triad {
    type Error = String;
    fn try_from(data: (i64, i64, i64)) -> Result<Triad, Self::Error> {
        let notes: (Note, Note, Note) = (data.0.into(), data.1.into(), data.2.into());
        Triad::try_from(notes)
    }
}

impl From<Triad> for (i64, i64, i64) {
    fn from(d: Triad) -> (i64, i64, i64) {
        (d.0.into(), d.1.into(), d.2.into())
    }
}

impl TryFrom<(Note, Note, Note)> for Triad {
    type Error = String;
    fn try_from(data: (Note, Note, Note)) -> Result<Triad, Self::Error> {
        if let Some(triad) = create_triad(data) {
            return Ok(triad);
        }
        Err("The provided notes don't contain the required relationships...".to_string())
    }
}

impl From<Triad> for (Note, Note, Note) {
    fn from(d: Triad) -> (Note, Note, Note) {
        (d.0.clone(), d.1.clone(), d.2.clone())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triad() {
        let a = Triads::Major.create(0.into());
        let tmp: (i64, i64, i64) = a.clone().into();
        assert_eq!(tmp, (0, 4, 7));
        let b = Triad::try_from((11, 4, 7));

        assert!(b.is_ok());
        assert_ne!(a, b.unwrap())
    }
}
