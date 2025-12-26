/*
    Appellation: wrapper <module>
    Contrib: @FL03
*/

#[macro_export]
macro_rules! wrapper {
    ($($S:ident($vis:vis $T:ident) $(where $($rest:tt)*)?;),* $(,)?) => {
        $(
            $crate::wrapper!(@impl $S($vis $T) $(where $($rest)*)?;);
        )*
    };
    (@impl
        #[derive($($derive:ident),*)]
        $S:ident($vis:vis $T:ident) $(where $($rest:tt)*)?;
    ) => {
        #[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd, $($derive),*)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize),
            serde(default, transparent),
        )]
        #[repr(transparent)]
        pub struct $S<$T>($vis $T) $(where $($rest)*)?;

        impl<$T> $S<$T> {
            /// returns a new instance with the given value
            pub const fn new(value: $T) -> Self {
                Self(value)
            }
            /// returns an immutable reference to the inner value
            pub const fn get(&self) -> &$T {
                &self.0
            }
            /// returns a mutable reference to the inner value
            pub const fn get_mut(&mut self) -> &mut $T {
                &mut self.0
            }
            /// consumes the current instance to return the inner value
            #[inline]
            pub fn into_inner(self) -> $T {
                self.0
            }
            /// applies the given function to the inner value and returns a new instance with
            /// the result
            #[inline]
            pub fn map<R, F>(self, f: F) -> $S<R>
            where
                F: FnOnce($T) -> R,
            {
                $S(f(self.0))
            }
            /// [`replace`](core::mem::replace) the inner value with the given, returning the previous value
            pub const fn replace(&mut self, value: $T) -> $T {
                ::core::mem::replace(self.get_mut(), value)
            }
            /// set the inner value, in-place
            #[inline]
            pub fn set(&mut self, value: $T) {
                *self.get_mut() = value;
            }
            /// [`swap`](core::mem::swap) the inner value with that of another instance of the same type
            pub const fn swap(&mut self, other: &mut Self) {
                ::core::mem::swap(self.get_mut(), other.get_mut());
            }
            /// [`take`](core::mem::take) the inner value, leaving a default in its place
            #[inline]
            pub fn take(&mut self) -> $T
            where
                $T: Default,
            {
                ::core::mem::take(self.get_mut())
            }
            /// consumes the current instance to create another with the given value
            pub fn with<_U>(self, value: _U) -> $S<_U> {
                $S::new(value)
            }
            /// captures a referenced value in a new instance
            pub fn view(&self) -> $S<&$T> {
                $S::new(self.get())
            }
            /// captures a mutable reference to the inner value
            pub fn view_mut(&mut self) -> $S<&mut $T> {
                $S::new(self.get_mut())
            }
        }

        impl<$T> AsRef<$T> for $S<$T> {
            fn as_ref(&self) -> &$T {
                self.get()
            }
        }

        impl<$T> AsMut<$T> for $S<$T> {
            fn as_mut(&mut self) -> &mut $T {
                self.get_mut()
            }
        }

        impl<$T> ::core::borrow::Borrow<$T> for $S<$T> {
            fn borrow(&self) -> &$T {
                self.get()
            }
        }

        impl<$T> ::core::borrow::BorrowMut<$T> for $S<$T> {
            fn borrow_mut(&mut self) -> &mut $T {
                self.get_mut()
            }
        }

        impl<$T> ::core::ops::Deref for $S<$T> {
            type Target = $T;

            fn deref(&self) -> &Self::Target {
                self.get()
            }
        }

        impl<$T> ::core::ops::DerefMut for $S<$T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.get_mut()
            }
        }

        impl<$T> From<$T> for $S<$T> {
            fn from(value: $T) -> Self {
                Self(value)
            }
        }
    };
}
