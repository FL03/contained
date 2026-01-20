/*
    Appellation: get <module>
    Created At: 2026.01.20:14:42:54
    Contrib: @FL03
*/

/// The [`Get`] trait establishes a basic interface for objects to retrieve an immutable
/// reference to an inner value of type `T`.
pub trait Get<T> {
    /// returns an immutable reference to the inner value
    fn get(&self) -> &T;
}
