/*
    appellation: impl_unary <module>
    authors: @FL03
*/
use crate::ast::{MethodCallAst, WrapperImpls};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// Procedural macro entry point
pub fn impl_wrapper_unary_ops(input: WrapperImpls) -> TokenStream {
    let base = impl_core_unary_ops(&input);

    quote! {
        #(#base)*

    }
}

fn impl_core_unary_ops(
    WrapperImpls {
        target, field, ops, ..
    }: &WrapperImpls,
) -> Vec<TokenStream> {
    let mut impls = Vec::new();
    for MethodCallAst { name: op, call, .. } in ops {
        let _impl = if let Some(f) = field {
            impl_named(op, target, call, f)
        } else {
            impl_tuple(op, target, call)
        };
        impls.push(_impl);
    }
    impls
}

fn impl_tuple(op: &Ident, target: &Ident, call: &Ident) -> TokenStream {
    quote! {
        impl<_A, _B> ::core::ops::#op for #target<_A>
        where
            _A: ::core::ops::#op<Output = _B>,
        {
            type Output = #target<_B>;

            fn #call(self) -> Self::Output {
                #target(::core::ops::#op::#call(self.0))
            }
        }

        impl<'a, _A, _B> ::core::ops::#op for &'a #target<_A>
        where
            &'a _A: ::core::ops::#op<Output = _B>,
        {
            type Output = #target<_B>;

            fn #call(self) -> Self::Output {
                #target(::core::ops::#op::#call(&self.0))
            }
        }

        impl<'a, _A, _B> ::core::ops::#op for &'a mut #target<_A>
        where
            &'a mut _A: ::core::ops::#op<Output = _B>,
        {
            type Output = #target<_B>;

            fn #call(self) -> Self::Output {
                #target(::core::ops::#op::#call(&mut self.0))
            }
        }
    }
}

fn impl_named(op: &Ident, target: &Ident, call: &Ident, field: &Ident) -> TokenStream {
    quote! {
        impl<_A, _B> ::core::ops::#op for #target<_A>
        where
            _A: ::core::ops::#op<Output = _B>,
        {
            type Output = #target<_B>;

            fn #call(self) -> Self::Output {
                let #field = ::core::ops::#op::#call(self.#field);
                #target { #field }
            }
        }

        impl<'a, _A, _B> ::core::ops::#op for &'a #target<_A>
        where
            &'a _A: ::core::ops::#op<Output = _B>,
        {
            type Output = #target<_B>;

            fn #call(self) -> Self::Output {
                let #field = ::core::ops::#op::#call(&self.#field);
                #target { #field }
            }
        }

        impl<'a, _A, _B> ::core::ops::#op for &'a mut #target<_A>
        where
            &'a mut _A: ::core::ops::#op<Output = _B>,
        {
            type Output = #target<_B>;

            fn #call(self) -> Self::Output {
                let #field = ::core::ops::#op::#call(&mut self.#field);
                #target { #field }
            }
        }
    }
}
