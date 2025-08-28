/*
    appellation: wrapper_ops <module>
    authors: @FL03
*/

/// the [`impl_wrapper_unary!`] macro implements binary operations for a wrapper type.
///
/// ## Syntax
///
/// For tuple structs:
///
/// ```ignore
/// impl_wrapper_unary! {
///    ${struct} { Op1.call, Op2.call, ... }
/// }
/// ```
///
/// For structs with named fields:
///
/// ```ignore
/// impl_wrapper_unary! {
///     ${struct}.${field} { Op1.call, Op2.call, ... }
/// }
/// ```
/// 
/// **Note**: The target struct must have exactly one field;
#[macro_export]
macro_rules! impl_wrapper_unary {
    ($s:ident.$field:ident { $($op:ident.$call:ident),* $(,)?}) => {
        $(
            $crate::impl_wrapper_unary!(@impl $s::$op.$call($field));
        )*
    };
    ($s:ident { $($op:ident.$call:ident),* $(,)?}) => {
        $(
            $crate::impl_wrapper_unary!(@impl $s::$op.$call(0));
        )*
    };
    (@impl $s:ident::$op:ident.$call:ident($field:ident)) => {
        impl<_A, _B> ::core::ops::$op for $s<_A>
        where
           _A: ::core::ops::$op<Output = _B>,
        {
            type Output = $s<_B>;

            fn $call(self) -> Self::Output {
                let $field = ::core::ops::$op::$call(self.$field);
                $s { $field }
            }
        }

        impl<'a, _A, _B> ::core::ops::$op for &'a $s<_A>
        where
            &'a _A: ::core::ops::$op<Output = _B>,
        {
            type Output = $s<_B>;

            fn $call(self) -> Self::Output {
                let $field = ::core::ops::$op::$call(&self.$field);
                $s { $field }
            }
        }

        impl<'a, _A, _B> ::core::ops::$op for &'a mut $s<_A>
        where
            &'a mut _A: ::core::ops::$op<Output = _B>,
        {
            type Output = $s<_B>;

            fn $call(self) -> Self::Output {
                let $field = ::core::ops::$op::$call(&mut self.$field);
                $s { $field }
            }
        }
    };
    (@impl $s:ident::$op:ident.$call:ident(0)) => {
        impl<_A, _B> ::core::ops::$op for $s<_A>
        where
           _A: ::core::ops::$op<Output = _B>,
        {
            type Output = $s<_B>;

            fn $call(self) -> Self::Output {
                $s(::core::ops::$op::$call(self.0))
            }
        }

        impl<'a, _A, _B> ::core::ops::$op for &'a $s<_A>
        where
            &'a _A: ::core::ops::$op<Output = _B>,
        {
            type Output = $s<_B>;

            fn $call(self) -> Self::Output {
                $s(::core::ops::$op::$call(&self.0))
            }
        }

        impl<'a, _A, _B> ::core::ops::$op for &'a mut $s<_A>
        where
            &'a mut _A: ::core::ops::$op<Output = _B>,
        {
            type Output = $s<_B>;

            fn $call(self) -> Self::Output {
                $s(::core::ops::$op::$call(&mut self.0))
            }
        }
    };
}
