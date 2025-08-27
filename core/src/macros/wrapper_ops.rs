/*
    appellation: wrapper_ops <module>
    authors: @FL03
*/

/// the [`impl_wrapper_binary!`] macro implements binary operations for a wrapper type.
///
/// ## Syntax
///
/// For tuple structs:
///
/// ```ignore
/// impl_wrapper_binary! {
///     WrapperType::<[Op1.call, Op2.call, ...]>
/// }
/// ```
///
/// For structs with named fields:
///
/// ```ignore
/// impl_wrapper_binary! {
///     ${struct}.$field::<[Op1.call, Op2.call, ...]>
/// }
/// ```
#[macro_export]
macro_rules! impl_wrapper_binary {
    ($s:ident.$field:tt { $($op:ident.$call:ident),* $(,)? }) => {
        $(
            $crate::impl_wrapper_binary!(@impl $s::$op.$call($field));
            $crate::impl_wrapper_binary!(@mut $s::$op.$call($field));
        )*
    };
    ($s:ident$(.$field:tt)? { $($op:ident.$call:ident),* $(,)? }) => {
        $(
            $crate::impl_wrapper_binary!(@impl $s::$op.$call(0));
            $crate::impl_wrapper_binary!(@mut $s::$op.$call(0));
        )*
    };
    (@impl $s:ident::$op:ident.$call:ident($field:tt)) => {
        impl<_A, _B, _C> ::core::ops::$op<$s<_B>> for $s<_A>
        where
            _A: ::core::ops::$op<_B, Output = _C>,
        {
            type Output = $s<_C>;

            fn $call(self, rhs: $s<_B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.$field, rhs.$field))
            }
        }

        impl<'a, _A, _B, _C> ::core::ops::$op<$s<_B>> for &'a $s<_A>
        where
            &'a _A: ::core::ops::$op<_B, Output = _C>,
        {
            type Output = $s<_C>;

            fn $call(self, rhs: $s<_B>) -> Self::Output {
                $s(::core::ops::$op::$call(&self.$field, rhs.$field))
            }
        }

        impl<'a, _A, _B, _C> ::core::ops::$op<&'a $s<_B>> for &'a $s<_A>
        where
            &'a _A: ::core::ops::$op<&'a _B, Output = _C>,
        {
            type Output = $s<_C>;

            fn $call(self, rhs: &'a $s<_B>) -> Self::Output {
                $s(::core::ops::$op::$call(&self.$field, &rhs.$field))
            }
        }

        impl<'a, _A, _B, _C> ::core::ops::$op<&'a $s<_B>> for $s<_A>
        where
            _A: ::core::ops::$op<&'a _B, Output = _C>,
        {
            type Output = $s<_C>;

            fn $call(self, rhs: &'a $s<_B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.$field, &rhs.$field))
            }
        }

        impl<'a, _A, _B, _C> ::core::ops::$op<$s<_B>> for &'a mut $s<_A>
        where
            &'a _A: ::core::ops::$op<_B, Output = _C>,
        {
            type Output = $s<_C>;

            fn $call(self, rhs: $s<_B>) -> Self::Output {
                $s(::core::ops::$op::$call(&self.$field, rhs.$field))
            }
        }

        impl<'a, _A, _B, _C> ::core::ops::$op<&'a mut $s<_B>> for $s<_A>
        where
            _A: ::core::ops::$op<&'a _B, Output = _C>,
        {
            type Output = $s<_C>;

            fn $call(self, rhs: &'a mut $s<_B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.$field, &rhs.$field))
            }
        }

        impl<'a, _A, _B, _C> ::core::ops::$op<&'a mut $s<_B>> for &'a mut $s<_A>
        where
            &'a _A: ::core::ops::$op<&'a _B, Output = _C>,
        {
            type Output = $s<_C>;

            fn $call(self, rhs: &'a mut $s<_B>) -> Self::Output {
                $s(::core::ops::$op::$call(&self.$field, &rhs.$field))
            }
        }
    };
    (@mut $s:ident::$op:ident.$call:ident($field:tt)) => {
        paste::paste! {
            $crate::impl_wrapper_binary_mut!(@impl $s::[<$op Assign>].[<$call _assign>]($field));
        }
    };
}
#[macro_export]
macro_rules! impl_wrapper_binary_mut {
    ($s:ident::<[$($op:ident.$call:ident),* $(,)?]>) => {
        $(
            $crate::impl_wrapper_binary_mut!(@impl $s::$op.$call);
        )*
    };
    (@impl $s:ident::$op:ident.$call:ident($field:tt)) => {
        impl<_A, _B> ::core::ops::$op<$s<_B>> for &mut $s<_A>
        where
            _A: ::core::ops::$op<_B>,
        {

            fn $call(&mut self, rhs: $s<_B>) {
                ::core::ops::$op::$call(&mut self.$field, rhs.$field)
            }
        }
    };
}
#[macro_export]
macro_rules! impl_wrapper_unary {
    ($s:ident::<[$($op:ident.$call:ident),* $(,)?]>) => {
        $(
            $crate::impl_wrapper_unary!(@impl $s::$op.$call(0));
        )*
    };
    (@impl $s:ident::$op:ident.$call:ident($field:tt)) => {
        impl<_A, _B> ::core::ops::$op for $s<_A>
        where
           _A: ::core::ops::$op<Output = _B>,
        {
            type Output = $s<_B>;

            fn $call(self) -> Self::Output {
                $s(core::ops::$op::$call(self.$field))
            }
        }

        impl<'a, _A, _B> ::core::ops::$op for &'a $s<_A>
        where
            &'a _A: ::core::ops::$op<Output = _B>,
        {
            type Output = $s<_B>;

            fn $call(self) -> Self::Output {
                $s(core::ops::$op::$call(&self.$field))
            }
        }

        impl<'a, _A, _B> ::core::ops::$op for &'a mut $s<_A>
        where
            &'a mut _A: ::core::ops::$op<Output = _B>,
        {
            type Output = $s<_B>;

            fn $call(self) -> Self::Output {
                $s(core::ops::$op::$call(&mut self.$field))
            }
        }

        impl<'a, _A, _B> ::core::ops::$op for $s<&'a A>
        where
            &'a _A: ::core::ops::$op<Output = _B>,
        {
            type Output = $s<_B>;

            fn $call(self) -> Self::Output {
                $s(core::ops::$op::$call(self.$field))
            }
        }

        impl<'a, _A, _B> ::core::ops::$op for $s<&'a mut A>
        where
            &'a mut _A: ::core::ops::$op<Output = _B>,
        {
            type Output = $s<_B>;

            fn $call(self) -> Self::Output {
                $s(core::ops::$op::$call(self.$field))
            }
        }
    };
}
