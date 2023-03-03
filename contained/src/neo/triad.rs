/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A triad is a set of three notes, called chord factors: root, third, and fifth
        Generaically, triad's share two of its notes with three of its inversions.

        For our purposes, a triad is said to be a three-tuple (a, b, c) where both [a, b] and [b, c] are thirds.
*/
use super::LPR;
use crate::actors::{
    turing::{Configuration, Machine, Program, Tape},
    Symbolic,
};
use crate::core::{Fifths, Note, Thirds};
use crate::core::{Gradient, Notable};
use crate::Resultant;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, EnumVariantNames};

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

impl<N: Notable> TryFrom<Triad<N>> for Triads {
    type Error = String;

    fn try_from(triad: Triad<N>) -> Result<Self, Self::Error> {
        let ab = Thirds::try_from((triad.root(), triad.third()))?;

        if Fifths::Perfect * triad.root() == triad.fifth() {
            let res = match ab {
                Thirds::Major => Self::Major,
                Thirds::Minor => Self::Minor,
            };
            return Ok(res);
        } else {
            if Fifths::Augmented * triad.root() == triad.fifth() && ab == Thirds::Major {
                return Ok(Self::Augmented);
            } else if Fifths::Diminished * triad.root() == triad.fifth() && ab == Thirds::Minor {
                return Ok(Self::Diminshed);
            }
            Err("Failed to find the required relationships...".to_string())
        }
    }
}

/// [Triad] is a set of three [Note], the root, third, and fifth.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Triad<N: Notable>(N, N, N);

impl<N: Notable> Triad<N> {
    pub fn new(root: N, class: Triads) -> Self {
        let (a, b) = Thirds::compute(root.clone());

        let triad = match class {
            Triads::Augmented => (root, a.clone(), Thirds::Major * a),
            Triads::Diminshed => (root, b.clone(), Thirds::Minor * b),
            Triads::Major => (root, a.clone(), Thirds::Minor * a),
            Triads::Minor => (root, b.clone(), Thirds::Major * b),
        };
        Self(triad.0, triad.1, triad.2)
    }
    ///
    pub fn classify(&self) -> Resultant<Triads> {
        Triads::try_from(self.clone())
    }
    /// Create a new [Configuration] with the [Triad] as its alphabet
    pub fn config(&self) -> Configuration<Note> {
        let a = self
            .clone()
            .into_iter()
            .map(|v| v.pitch().into())
            .collect::<Vec<Note>>();
        Configuration::build(Tape::new(a), None)
    }
    /// Repeatedly applies a chain of transformations to the [Triad]
    pub fn cycle(&mut self, cycle: impl IntoIterator<Item = LPR> + Clone) {
        loop {
            self.walk(Vec::from_iter(cycle.clone()));
        }
    }
    /// Tries to create a [Machine] running the given [Program] with a default set to the triad's root
    pub fn machine(&self, program: Program<Note>) -> Resultant<Machine<Note>> {
        Machine::new(self.root().pitch().into(), program)
    }
    /// Check and see if the given notes are classified by the defined relationships
    pub fn is_valid(&self) -> bool {
        self.classify().is_ok()
    }
    ///
    pub fn fifth(&self) -> N {
        self.2.clone()
    }
    ///
    pub fn root(&self) -> N {
        self.0.clone()
    }
    ///
    pub fn third(&self) -> N {
        self.1.clone()
    }
    /// Apply a single [LPR] transformation onto the active machine
    /// For convenience, [std::ops::Mul] was implemented as a means of applying the transformation
    pub fn transform(&mut self, dirac: LPR) {
        *self = dirac.transform(self);
    }
    /// Applies multiple [LPR] transformations onto the scoped [Triad]
    /// The goal here is to allow the machine to work on and in the scope
    pub fn walk(&mut self, chain: impl IntoIterator<Item = LPR>) {
        for dirac in chain {
            self.transform(dirac);
        }
    }
}

impl<N: Eq + Notable + Ord + Serialize + std::fmt::Debug> Symbolic for Triad<N> {}

impl<N: Notable> std::fmt::Display for Triad<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
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

impl<N: Notable> std::ops::MulAssign<LPR> for Triad<N> {
    fn mul_assign(&mut self, rhs: LPR) {
        self.transform(rhs);
    }
}

impl<N: Notable> IntoIterator for Triad<N> {
    type Item = N;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.0, self.1, self.2].into_iter()
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
                if Thirds::try_from((a.clone(), b.clone())).is_ok()
                    && Thirds::try_from((b.clone(), c.clone())).is_ok()
                {
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

    #[test]
    fn test_cycles() {
        let triad = Triad::<Note>::new(0.into(), Triads::Major);
        let mut a = triad.clone();
        a *= LPR::L;
        assert_eq!(a.clone(), Triad::try_from((11, 4, 7)).unwrap());
        a *= LPR::L;
        assert_eq!(a.clone(), triad.clone());
        a.walk(vec![LPR::L, LPR::L]);
        assert_eq!(a.clone(), triad)
    }
}
