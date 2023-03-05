/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

/// [absmod] is short for the absolute value of a modular number;
pub fn absmod(a: i64, m: i64) -> i64 {
    (((a % m) + m) % m).abs()
}

/// [harmonic_transformation] is a transformative function for continuous musical space
/// This is useful for describing the behavior between transitions as nothing is achieved instantly
pub fn harmonic_transformation(a: usize, b: usize, t: usize) -> usize {
    (b - a) * t + a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_absmod() {
        let a: i64 = -1 % 12;
        assert_ne!(a.abs(), absmod(-1, 12));
        assert_eq!(absmod(-1, 12), 11);
    }
}
