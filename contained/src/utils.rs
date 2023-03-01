/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

/// [harmonic_transformation] is a transformative function for continuous musical space
/// This is useful for describing the behavior between transitions as nothing is achieved instantly
pub fn harmonic_transformation(a: usize, b: usize, t: usize) -> usize {
    (b - a) * t + a
}

/// [absmod] is short for the absolute value of a modular number;
pub fn absmod(a: i64, m: i64) -> i64 {
    (((a % m) + m) % m).abs()
}