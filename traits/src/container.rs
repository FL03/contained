/*
    Appellation: store <module>
    Created At: 2025.12.26:14:12:46
    Contrib: @FL03
*/
use crate::store::RawStore;

/// The [`Container`] trait is a higher-kinded trait used to establish an interface for
/// defining containers themselves.
pub trait Container<U>
where
    Self::Cont<U>: RawStore<Elem = U>,
{
    type Cont<V>;
}
