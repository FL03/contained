/*
    Appellation: store <module>
    Created At: 2025.12.26:14:12:46
    Contrib: @FL03
*/
/// The [`RawStore`] trait is used to define a base interface for all containers whose elements
/// are of **one** specific type.
pub trait RawStore {
    /// The type of elements associated with the container
    type Elem;
}

/*
 ************* Implementations *************
*/

impl<C, T> RawStore for &C
where
    C: RawStore<Elem = T>,
{
    type Elem = C::Elem;
}

impl<C, T> RawStore for &mut C
where
    C: RawStore<Elem = T>,
{
    type Elem = C::Elem;
}

macro_rules! impl_raw_store  {
    (impl<Elem = $elem:ident> $trait:ident for {$(
        $($cont:ident)::*<$($T:ident),*> $({where $($rest:tt)*})?
    ),* $(,)?}) => {
        $(impl_raw_store! {
            @impl<Elem = $elem> $trait for $($cont)::*<$($T),*> $(where $($rest)*)?
        })*
    };
    (@impl<Elem = $elem:ident> $trait:ident for $($cont:ident)::*<$($T:ident),*> $(where $($rest:tt)*)?) => {
        impl<$($T),*> $trait for $($cont)::*<$($T),*> $(where $($rest)*)? {
            type Elem = $elem;
        }
    };
}

macro_rules! impl_raw_tuple_store {
    (@impl<$T:ident> $trait:ident for ($($name:ident),+ $(,)?)) => {
        impl<$T> $trait for ($($name),+) {
            type Elem = $T;
        }
    };
    (impl<$T:ident> $trait:ident for {$(($($name:ident),+)),* $(,)?}) => {
        $(impl_raw_tuple_store! { @impl<$T> $trait for ($($name),+) } )*
    };
}

impl_raw_tuple_store! {
    impl<T> RawStore for {
        (T, T),
        (T, T, T),
        (T, T, T, T),
        (T, T, T, T, T),
        (T, T, T, T, T, T),
        (T, T, T, T, T, T, T),
        (T, T, T, T, T, T, T, T),
        (T, T, T, T, T, T, T, T, T),
        (T, T, T, T, T, T, T, T, T, T),
    }
}

impl_raw_store! {
    impl<Elem = T> RawStore for {
        core::option::Option<T>,
        core::cell::Cell<T>,
        core::cell::OnceCell<T>,
        core::cell::RefCell<T>,
        core::cell::UnsafeCell<T>,
        core::ops::Range<T>,
        core::result::Result<T, E>,
    }
}

#[cfg(feature = "alloc")]
impl_raw_store! {
    impl<Elem = T> RawStore for {
        alloc::boxed::Box<T>,
        alloc::rc::Rc<T>,
        alloc::sync::Arc<T>,
        alloc::vec::Vec<T>,
        alloc::collections::BTreeSet<T>,
        alloc::collections::LinkedList<T>,
        alloc::collections::VecDeque<T>,
        alloc::collections::BinaryHeap<T>,
        alloc::collections::BTreeMap<K, T>,
    }
}

#[cfg(feature = "std")]
impl_raw_store! {
    impl<Elem = T> RawStore for {
        std::sync::Mutex<T>,
        std::sync::RwLock<T>,
        std::sync::LazyLock<T>,
        std::collections::HashMap<K, T>,
        std::collections::HashSet<T>,
    }
}

#[cfg(feature = "hashbrown")]
impl_raw_store! {
    impl<Elem = T> RawStore for {
        hashbrown::HashMap<K, T, S>,
        hashbrown::HashSet<T, S>,
    }
}

impl<T> RawStore for [T] {
    type Elem = T;
}

impl<T> RawStore for &[T] {
    type Elem = T;
}

impl<T> RawStore for &mut [T] {
    type Elem = T;
}

impl<const N: usize, T> RawStore for [T; N] {
    type Elem = T;
}
