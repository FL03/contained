/*
    Appellation: wrapper <module>
    Created At: 2026.01.20:14:40:06
    Contrib: @FL03
*/

/// The [`Wrapper`] trait established a higher-kinded interface for single-value containers.
pub trait Wrapper<T> {
    type Cont<U>: ?Sized;

    /// returns a reference to the inner value
    fn get(&self) -> &T;
    /// returns a view of the container containing an immutable reference to the inner value
    fn view(&self) -> Self::Cont<&T>;
}
/// The [`WrapperMut`] trait extends the [`Wrapper`] trait to provide mutable access to the
/// inner value.
pub trait WrapperMut<T>: Wrapper<T> {
    /// returns a mutable reference to the inner value
    fn get_mut(&mut self) -> &mut T;
    /// returns a view of the container containing a mutable reference to the inner value
    fn view_mut(&mut self) -> Self::Cont<&mut T>;
}

impl<T> Wrapper<T> for T {
    type Cont<U> = U;

    fn get(&self) -> &T {
        self
    }

    fn view(&self) -> Self::Cont<&T> {
        self
    }
}
