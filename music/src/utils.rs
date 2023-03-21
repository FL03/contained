/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use itertools::Itertools;

/// [absmod] is short for the absolute value of a modular number;
pub fn absmod(a: i64, m: i64) -> i64 {
    (((a % m) + m) % m).abs()
}

/// [harmonic_transformation] is a transformative function for continuous musical space
/// This is useful for describing the behavior between transitions as nothing is achieved instantly
pub fn harmonic_transformation(a: usize, b: usize, t: usize) -> usize {
    (b - a) * t + a
}

/// Find the difference between a collection of items where each element implements [Clone], [Into<i64>], and [Ord]
pub fn intervals<T>(args: impl IntoIterator<Item = T>) -> Vec<((T, T), i64)>
where
    T: Clone + Into<i64> + Ord,
{
    let pairs = {
        let mut tmp = Vec::from_iter(args);
        tmp.sort();
        tmp.clone()
            .into_iter()
            .circular_tuple_windows::<(T, T)>()
            .collect::<Vec<_>>()
    };
    let mut res = pairs
        .into_iter()
        .map(|i| (i.clone(), i.1.into() - i.0.into()))
        .collect::<Vec<_>>();
    res.sort_by(|a, b| {
        a.0.partial_cmp(&b.0)
            .unwrap()
            .then(a.1.abs().partial_cmp(&b.1.abs()).unwrap())
    });
    res
}
/// A simple function wrapper for [Itertools::permutations]
pub fn permute<T>(args: impl IntoIterator<Item = T>, size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    args.into_iter().permutations(size).collect::<Vec<_>>()
}
