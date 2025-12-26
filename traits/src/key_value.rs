/*
    Appellation: key_value <module>
    Created At: 2025.12.26:17:44:22
    Contrib: @FL03
*/
//! this module defines traits for key-value stores and their entries
//!

/// [`KeyValueEntry`] establishes a common interface for entries within a key-value store.
pub trait KeyValueEntry<'a> {
    type Key;
    type Value;
}

/// The [`KeyValueStore`] trait is used to define a key-value store container.
pub trait KeyValueStore<K, V> {
    type Entry<'a>: KeyValueEntry<'a, Key = K, Value = V>
    where
        Self: 'a;
}

/*
 ************* Implementations *************
*/

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::{KeyValueEntry, KeyValueStore};
    use alloc::collections::btree_map::{self, BTreeMap};

    impl<'a, K, V> KeyValueEntry<'a> for btree_map::Entry<'a, K, V> {
        type Key = K;
        type Value = V;
    }

    impl<K, V> KeyValueStore<K, V> for BTreeMap<K, V> {
        type Entry<'a>
            = btree_map::Entry<'a, K, V>
        where
            Self: 'a;
    }
}

#[cfg(feature = "hashbrown")]
mod impl_hashbrown {
    use super::{KeyValueEntry, KeyValueStore};
    use hashbrown::hash_map::{self, HashMap};

    impl<'a, K, V, S> KeyValueEntry<'a> for hash_map::Entry<'a, K, V, S> {
        type Key = K;
        type Value = V;
    }

    impl<K, V, S> KeyValueStore<K, V> for HashMap<K, V, S> {
        type Entry<'a>
            = hash_map::Entry<'a, K, V, S>
        where
            Self: 'a;
    }
}

#[cfg(feature = "std")]
mod impl_std {
    use super::{KeyValueEntry, KeyValueStore};
    use std::collections::hash_map::{self, HashMap};

    impl<'a, K, V> KeyValueEntry<'a> for hash_map::Entry<'a, K, V> {
        type Key = K;
        type Value = V;
    }

    impl<K, V> KeyValueStore<K, V> for HashMap<K, V> {
        type Entry<'a>
            = hash_map::Entry<'a, K, V>
        where
            Self: 'a;
    }
}
