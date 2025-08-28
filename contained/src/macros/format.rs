/*
    appellation: format <module>
    authors: @FL03
*/

/// A macro to implement formatting traits for wrapper structs
///
/// For tuple structs, use the following:
/// 
/// ```ignore
/// fmt_wrapper! {
///     impl WrapperType<T> {
///         Display, 
///         Debug, 
///         ...
///     }
/// }
/// ```
///
/// For structs with named fields, use the following syntax, replacing `field` with the actual field name:
///
/// ```ignore
/// fmt_wrapper! {
///     impl WrapperType<T>.field {
///         Display, 
///         Debug, 
///         ...
///     }
/// }
/// ```
#[macro_export]
macro_rules! fmt_wrapper {
    (impl $s:ident<$T:ident>.$field:ident { $($trait:ident),* $(,)? }) => {
        $(
            $crate::fmt_wrapper!(@impl $s<$T>::$trait.$field);
        )*
    };
    (impl $s:ident<$T:ident>$(.$field:ident)? { $($trait:ident),* $(,)? }) => {
        $(
            $crate::fmt_wrapper!(@impl $s<$T>::$trait.0);
        )*
    };
    (@impl $s:ident<$T:ident>::$trait:ident.$field:tt) => {
        impl<$T> ::core::fmt::$trait for $s<$T>
        where
            $T: ::core::fmt::$trait
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::$trait::fmt(
                    &self.$field,
                    f,
                )
            }
        }
    };
}
