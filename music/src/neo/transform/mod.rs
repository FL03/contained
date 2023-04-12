/*
    Appellation: transform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

pub use self::{lpr::LPR, transformer::*};

mod lpr;
mod transformer;

/// [Dirac] is a trait used to describe a transformative function;
/// Often, this trait is used to describe a set of functions that are used to transform one object into another of the same type.
pub trait Dirac<T> {
    type Output;
    /// The function that transforms the object
    fn dirac(&self, arg: &mut T) -> Self::Output;
}

/// [Transform] is a trait used to describe a type that can be transformed by a [Dirac] function.
pub trait Transform: Sized {
    type Dirac: Dirac<Self, Output = Self>;

    fn transform(&mut self, dirac: Self::Dirac) -> Self {
        dirac.dirac(self)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use futures::StreamExt;

    use super::*;
    use crate::neo::triads::*;

    #[test]
    fn test_transformer() {
        use LPR::*;
        let iter = vec![L, P, R];
        let triad = Triad::new(0.into(), Triads::Major);
        let mut transformer = Transformer::new(triad.clone()).with(iter.clone());
        let expected = {
            let mut tmp = triad.clone();
            tmp.walk(iter);
            tmp
        };
        assert_eq!(transformer.next().unwrap(), triad.clone() * L);
        assert_eq!(transformer.next().unwrap(), (triad * L) * P);
        assert_eq!(transformer.next().unwrap(), expected);
    }

    #[tokio::test]
    async fn test_stream_transformer() {
        use futures::stream;
        use LPR::*;
        let iter = vec![L, P, R];
        let triad = Triad::new(0.into(), Triads::Major);
        let transformer = Transformer::new(triad.clone()).with(iter.clone());
        let expected = {
            let mut tmp = triad.clone();
            tmp.walk(iter);
            tmp
        };
        let stream = stream::iter(transformer);
        let res = stream.collect::<Vec<_>>().await;
        assert_eq!(res[0], triad.clone() * L);
        assert_eq!(res[1], (triad * L) * P);
        assert_eq!(res[2], expected);
    }

    #[test]
    fn test_lpr() {
        assert_eq!(LPR::from_str("l"), LPR::from_str("leading"));
    }

    #[test]
    fn test_leading() {
        for cls in [Triads::Major] {
            let a = Triad::new(0.into(), cls);
            let b = LPR::L * a.clone();
            assert_ne!(a, b);
            assert_eq!(LPR::L * b, a);
        }
    }

    #[test]
    fn test_parallel() {
        for cls in [Triads::Major] {
            let a = Triad::new(0.into(), cls);
            let b = LPR::P * a.clone();
            assert_ne!(a, b);
            assert_eq!(LPR::P * b, a);
        }
    }

    #[test]
    fn test_relative() {
        for cls in [Triads::Major] {
            let a = Triad::new(0.into(), cls);
            let b = LPR::R * a.clone();
            assert_ne!(a, b);
            assert_eq!(LPR::R * b, a);
        }
    }
}
