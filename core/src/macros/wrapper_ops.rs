/*
    appellation: wrapper_ops <module>
    authors: @FL03
*/

/// the [`impl_wrapper_binary!`] macro implements binary operations for a wrapper type.
///
/// ## Syntax
///
/// ```no_run
/// impl_wrapper_binary!(WrapperType::<[Op1.call, Op2.call, ...]>);
/// ```
#[macro_export]
macro_rules! impl_wrapper_binary {
    ($s:ident::<[$($op:ident.$call:ident),* $(,)?]>) => {
        $(
            impl_wrapper_binary!(@impl $s::$op.$call);
            impl_wrapper_binary!(@mut $s::$op.$call);
        )*
    };
    (@impl $s:ident::$op:ident.$call:ident) => {
        impl<A, B, C> ::core::ops::$op<$s<B>> for $s<A>
        where
            A: ::core::ops::$op<B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.0, rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<$s<B>> for &'a $s<A>
        where
            &'a A: ::core::ops::$op<B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(&self.0, rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<&'a $s<B>> for &'a $s<A>
        where
            &'a A: ::core::ops::$op<&'a B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: &'a $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(&self.0, &rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<&'a $s<B>> for $s<A>
        where
            A: ::core::ops::$op<&'a B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: &'a $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.0, &rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<$s<B>> for &'a mut $s<A>
        where
            &'a A: ::core::ops::$op<B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(&self.0, rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<&'a mut $s<B>> for $s<A>
        where
            A: ::core::ops::$op<&'a B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: &'a mut $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(self.0, &rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$op<&'a mut $s<B>> for &'a mut $s<A>
        where
            &'a A: ::core::ops::$op<&'a B, Output = C>,
        {
            type Output = $s<C>;

            fn $call(self, rhs: &'a mut $s<B>) -> Self::Output {
                $s(::core::ops::$op::$call(&self.0, &rhs.0))
            }
        }
    };
    (@mut $s:ident::$op:ident.$call:ident) => {
        paste::paste! {
            impl_wrapper_binary_mut!(@impl $s::[<$op Assign>].[<$call _assign>]);
        }
    };
}
#[macro_export]
macro_rules! impl_wrapper_binary_mut {
    ($s:ident::<[$($op:ident.$call:ident),* $(,)?]>) => {
        $(
            impl_wrapper_binary_mut!(@impl $s::$op.$call);
        )*
    };
    (@impl $s:ident::$op:ident.$call:ident) => {
        impl<A, B> ::core::ops::$op<$s<B>> for &mut $s<A>
        where
            A: ::core::ops::$op<B>,
        {

            fn $call(&mut self, rhs: $s<B>) {
                core::ops::$op::$call(&mut self.0, rhs.0)
            }
        }
    };
}
#[macro_export]
macro_rules! impl_wrapper_unary {
    ($s:ident::<[$($op:ident.$call:ident),* $(,)?]>) => {
        $(
            impl_wrapper_unary!(@impl $s::$op.$call);
        )*
    };
    (@impl $s:ident::$op:ident.$call:ident) => {
        impl<A, B> ::core::ops::$op for $s<A>
        where
            A: ::core::ops::$op<Output = B>,
        {
            type Output = $s<B>;

            fn $call(self) -> Self::Output {
                $s(core::ops::$op::$call(self.0))
            }
        }

        impl<'a, A, B> ::core::ops::$op for &'a $s<A>
        where
            &'a A: ::core::ops::$op<Output = B>,
        {
            type Output = $s<B>;

            fn $call(self) -> Self::Output {
                $s(core::ops::$op::$call(&self.0))
            }
        }

        impl<'a, A, B> ::core::ops::$op for &'a mut $s<A>
        where
            &'a mut A: ::core::ops::$op<Output = B>,
        {
            type Output = $s<B>;

            fn $call(self) -> Self::Output {
                $s(core::ops::$op::$call(&mut self.0))
            }
        }
    };
}
