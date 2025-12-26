/*
    Appellation: transform <module>
    Created At: 2025.12.24:14:22:11
    Contrib: @FL03
*/

/// The [`Transform`] trait establishes a common interface for objects that can be transformed
/// with respect to a given transformation, input, etc. to produce a new output.
pub trait Transform<Rhs> {
    type Output;

    fn transform(&self, rhs: Rhs) -> Self::Output;
}

pub trait TryTransform<Rhs> {
    type Output;
    type Error;

    fn try_transform(&self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}
