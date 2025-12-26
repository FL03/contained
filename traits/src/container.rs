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
    type Cont<V>: ?Sized;
}

/*
 ************* Implementations *************
*/

impl<C, T> Container<T> for &C
where
    C: Container<T>,
    C::Cont<T>: RawStore<Elem = T>,
{
    type Cont<U> = <C>::Cont<U>;
}

impl<C, T> Container<T> for &mut C
where
    C: Container<T>,
    C::Cont<T>: RawStore<Elem = T>,
{
    type Cont<U> = <C>::Cont<U>;
}

impl<T> Container<T> for [T] {
    type Cont<U> = [U];
}

impl<T, const N: usize> Container<T> for [T; N] {
    type Cont<U> = [U; N];
}

#[allow(unused_macros)]
macro_rules! impl_container  {
    (impl<Elem = $E:ident> $trait:ident for {$(
        $($cont:ident)::*<$($T:ident),*> $({where $($rest:tt)*})?
    ),* $(,)?}) => {
        $(impl_container! {
            @impl<Elem = $E> $trait for $($cont)::*<$($T),*> $(where $($rest)*)?
        })*
    };
    (@impl<Elem = $E:ident> $trait:ident for $($cont:ident)::*<$($T:ident),*> $(where $($rest:tt)*)?) => {
        paste::paste! {
            impl<$($T),*> $trait<$E> for $($cont)::*<$($T),*> $(where $($rest)*)? {
                type Cont<$([<_ $T>]),*> = $($cont)::*<$([<_ $T>]),*>;
            }
        }
    };
}

macro_rules! impl_tuple_container {
    (@impl<Elem = $E:ident> $trait:ident for ($($name:ident),+ $(,)?)) => {
        paste::paste! {
            impl<$E> $trait<$E> for ($($name),+) {
                type Cont<[<_ $E>]> = ($([<_ $name>]),+);
            }
        }
    };
    (impl<$T:ident> $trait:ident for { $(($($name:ident),+ $(,)?)),* $(,)?}) => {
        $(impl_tuple_container! { @impl<Elem = $T> $trait for ($($name),+) })*
    };
}

impl_tuple_container! {
    impl<T> Container for {
        (T, T),
        (T, T, T),
        (T, T, T, T),
        (T, T, T, T, T),
        (T, T, T, T, T, T),
        (T, T, T, T, T, T, T),
        (T, T, T, T, T, T, T, T),
        (T, T, T, T, T, T, T, T, T),
        (T, T, T, T, T, T, T, T, T, T),
        (T, T, T, T, T, T, T, T, T, T, T),
        (T, T, T, T, T, T, T, T, T, T, T, T),
    }
}

// impl_container! {
//     impl<Elem = T> Container for {
//         core::option::Option<T>,
//         core::cell::Cell<T>,
//         core::cell::OnceCell<T>,
//         core::cell::RefCell<T>,
//         core::cell::UnsafeCell<T>,
//         core::ops::Range<T>,
//         core::result::Result<T, E>,
//     }
// }

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::Container;
    use alloc::collections::*;
    use alloc::vec::Vec;

    impl<T> Container<T> for Vec<T> {
        type Cont<U> = Vec<U>;
    }

    impl<V> Container<V> for BTreeSet<V> {
        type Cont<U> = BTreeSet<U>;
    }

    impl<K, V> Container<V> for BTreeMap<K, V>
    where
        K: Ord,
    {
        type Cont<U> = BTreeMap<K, U>;
    }

    impl<V> Container<V> for LinkedList<V> {
        type Cont<U> = LinkedList<U>;
    }

    impl<V> Container<V> for VecDeque<V> {
        type Cont<U> = VecDeque<U>;
    }
}
