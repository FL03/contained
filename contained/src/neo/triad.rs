/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A triad is a set of three notes, called chord factors: root, third, and fifth
        Generaically, triad's share two of its notes with three of its inversions.

        The neo-Riemannian theory highlightes three transformations, each of which preserve two of the three notes.

        We express a triad as a ordered three tuple <a, b, c> where a, b, c are integers modulus of 12 and:
            a != b
            a != c
            b != c
*/
use super::LPR;
use crate::actors::{
    turing::{Configuration, Machine, Program, Tape},
    Symbolic,
};
use crate::core::{is_major_third, is_minor_third, is_third, Chord, Fifths, Note, Thirds};
use crate::core::{Gradient, Notable};
use crate::Resultant;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

pub trait Triadic<N: Notable>: Clone {
    fn create(root: N, class: Triads) -> Self;
    /// [Triadic::chord] Creates a [Chord] from the vertices
    fn chord(&self) -> Chord {
        Chord::new(vec![
            self.root().pitch().into(),
            self.third().pitch().into(),
            self.fifth().pitch().into(),
        ])
    }
    /// [Triadic::classify] tries to define the triad by searching for triadic relations
    fn classify(&self) -> Resultant<Triads> {
        if Fifths::Perfect * self.root() == self.fifth() {
            if is_major_third(self.root(), self.third()) {
                return Ok(Triads::Major);
            } else {
                return Ok(Triads::Minor);
            }
        } else {
            if is_major_third(self.root(), self.third())
                && is_major_third(self.third(), self.fifth())
            {
                return Ok(Triads::Augmented);
            } else if is_minor_third(self.root(), self.third())
                && is_minor_third(self.third(), self.fifth())
            {
                return Ok(Triads::Diminshed);
            }
            Err("Failed to find the required relationships...".to_string())
        }
    }
    /// [Triadic::config] Create a new [Configuration] with the [Triad] as its alphabet
    fn config(&self) -> Configuration<Note> {
        Configuration::norm(Tape::new(self.chord())).unwrap()
    }
    /// [Triadic::machine] Tries to create a [Machine] running the given [Program] with a default set to the triad's root
    fn machine(&self, program: Program<Note>) -> Resultant<Machine<Note>> {
        Machine::new(self.root().pitch().into(), program)
    }
    /// [Triadic::is_valid] A method for establishing the validity of the given notes
    fn is_valid(&self) -> bool {
        self.classify().is_ok()
    }
    fn fifth(&self) -> N;
    fn root(&self) -> N;
    fn third(&self) -> N;
    fn triad(&self) -> &Self
    where
        Self: Sized,
    {
        &self
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
    Minor,     // If the root -> third is minor and if third -> fifth is major
}

/// [Triad] is a set of three [Note], the root, third, and fifth.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Triad<N: Notable>(N, N, N);

impl<N: Notable> Triad<N> {
    pub fn new(root: N, class: Triads) -> Self {
        Self::create(root, class)
    }
}

impl<N: Eq + Notable + Serialize> Symbolic for Triad<N> {}

impl<N: Notable> Triadic<N> for Triad<N> {
    fn create(root: N, class: Triads) -> Self {
        let (a, b) = Thirds::compute_both(root.clone());

        let triad = match class {
            Triads::Augmented => (root, a.clone(), Thirds::Major * a),
            Triads::Diminshed => (root, b.clone(), Thirds::Minor * b),
            Triads::Major => (root, a.clone(), Thirds::Minor * a),
            Triads::Minor => (root, b.clone(), Thirds::Major * b),
        };
        Self::try_from(triad).unwrap()
    }
    fn fifth(&self) -> N {
        self.2.clone()
    }

    fn root(&self) -> N {
        self.0.clone()
    }

    fn third(&self) -> N {
        self.1.clone()
    }
}

impl<N: Notable> std::fmt::Display for Triad<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.0.to_string(),
            self.1.to_string(),
            self.2.to_string()
        )
    }
}

impl<N: Notable> std::ops::Mul<LPR> for Triad<N> {
    type Output = Triad<N>;

    fn mul(self, rhs: LPR) -> Self::Output {
        rhs.transform(&mut self.clone())
    }
}

impl<N: Notable> IntoIterator for Triad<N> {
    type Item = N;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.0, self.1, self.2].into_iter()
    }
}

impl<N: Notable> TryFrom<(i64, i64, i64)> for Triad<N> {
    type Error = String;
    fn try_from(data: (i64, i64, i64)) -> Result<Triad<N>, Self::Error> {
        let notes: (N, N, N) = (
            data.0.pitch().into(),
            data.1.pitch().into(),
            data.2.pitch().into(),
        );
        Triad::try_from(notes)
    }
}

impl<N: Notable> From<Triad<N>> for (i64, i64, i64) {
    fn from(d: Triad<N>) -> (i64, i64, i64) {
        (d.0.pitch(), d.1.pitch(), d.2.pitch())
    }
}

impl<N: Notable> TryFrom<(N, N, N)> for Triad<N> {
    type Error = String;

    fn try_from(data: (N, N, N)) -> Result<Triad<N>, Self::Error> {
        let args = vec![data.0, data.1, data.2];
        for i in 0..args.len() {
            let tmp = [(i + 1) % args.len(), (i + 2) % args.len()];
            for j in 0..tmp.len() {
                let (a, b, c) = (
                    args[i].clone(),
                    args[tmp[j]].clone(),
                    args[tmp[(j + 1) % tmp.len()]].clone(),
                );
                // Creates a triad if the two intervals of [root, third], [third, fifth] are both considered thirds
                if is_third(a.clone(), b.clone()) && is_third(b.clone(), c.clone()) {
                    return Ok(Triad(a, b, c));
                }
            }
        }
        Err("Failed to find the required relationships within the given notes...".to_string())
    }
}

impl<N: Notable> From<Triad<N>> for (N, N, N) {
    fn from(d: Triad<N>) -> (N, N, N) {
        (d.0.clone(), d.1.clone(), d.2.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Note;

    #[test]
    fn test_triad() {
        let a = Triad::<Note>::new(0.into(), Triads::Major);
        let tmp: (i64, i64, i64) = a.clone().into();
        assert_eq!(tmp, (0, 4, 7));
        let b = Triad::try_from((11, 4, 7));

        assert!(b.is_ok());
        assert_ne!(a, b.unwrap())
    }
}
