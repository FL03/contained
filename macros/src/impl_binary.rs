/*
    appellation: impl_binary <module>
    authors: @FL03
*/
use crate::ast::WrapperOpsAst;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

/// Procedural macro entry point
pub fn impl_wrapper_binary_ops(input: WrapperOpsAst) -> TokenStream {
    let base = impl_core_binary_ops(&input);
    let assign = impl_assign_ops(&input);

    quote! {
        #(#base)*

        #(#assign)*
    }
}

fn impl_core_binary_ops(
    WrapperOpsAst {
        target, field, ops, ..
    }: &WrapperOpsAst,
) -> Vec<TokenStream> {
    let mut impls = Vec::new();
    for (op, call) in ops {
        let _impl = if let Some(f) = field {
            impl_named(op, target, call, f)
        } else {
            impl_unnamed(op, target, call)
        };
        impls.push(_impl);
    }
    impls
}

fn impl_assign_ops(options: &WrapperOpsAst) -> Vec<TokenStream> {
    let WrapperOpsAst {
        target, field, ops, ..
    } = options;

    let mut impls = Vec::new();
    for (op, call) in ops {
        let op_assign = format_ident!("{}Assign", op);
        let call_assign = format_ident!("{}_assign", call);

        let _impl = if let Some(f) = field {
            quote! {
                impl<_A, _B> ::core::ops::#op_assign<#target<_B>> for #target<_A>
                where
                    _A: ::core::ops::#op_assign<_B>,
                {
                    fn #call_assign(&mut self, rhs: #target<_B>) {
                        ::core::ops::#op_assign::#call_assign(&mut self.#f, rhs.#f)
                    }
                }
            }
        } else {
            quote! {
                impl<_A, _B> ::core::ops::#op_assign<#target<_B>> for #target<_A>
                where
                    _A: ::core::ops::#op_assign<_B>,
                {
                    fn #call_assign(&mut self, rhs: #target<_B>) {
                        ::core::ops::#op_assign::#call_assign(&mut self.0, rhs.0)
                    }
                }
            }
        };
        // register the implementation
        impls.push(_impl);
    }
    impls
}

fn impl_unnamed(op: &Ident, target: &Ident, call: &Ident) -> TokenStream {
    quote! {
        impl<_A, _B, _C> ::core::ops::#op<#target<_B>> for #target<_A>
        where
            _A: ::core::ops::#op<_B, Output = _C>,
        {
            type Output = #target<_C>;

            fn #call(self, rhs: #target<_B>) -> Self::Output {
                #target(::core::ops::#op::#call(self.0, rhs.0))
            }
        }

        impl<'a, _A, _B, _C> ::core::ops::#op<&'a #target<_B>> for #target<_A>
        where
            _A: ::core::ops::#op<&'a _B, Output = _C>,
        {
            type Output = #target<_C>;

            fn #call(self, rhs: &'a #target<_B>) -> Self::Output {
                #target(::core::ops::#op::#call(self.0, &rhs.0))
            }
        }

        impl<'a, _A, _B, _C> ::core::ops::#op<&'a #target<_B>> for &'a #target<_A>
        where
            &'a _A: ::core::ops::#op<&'a _B, Output = _C>,
        {
            type Output = #target<_C>;

            fn #call(self, rhs: &'a #target<_B>) -> Self::Output {
                #target(::core::ops::#op::#call(&self.0, &rhs.0))
            }
        }

        impl<'a, _A, _B, _C> ::core::ops::#op<#target<_B>> for &'a #target<_A>
        where
            &'a _A: ::core::ops::#op<_B, Output = _C>,
        {
            type Output = #target<_C>;

            fn #call(self, rhs: #target<_B>) -> Self::Output {
                #target(::core::ops::#op::#call(&self.0, rhs.0))
            }
        }

        impl<'a, _A, _B, _C> ::core::ops::#op<&'a mut #target<_B>> for #target<_A>
        where
            _A: ::core::ops::#op<&'a mut _B, Output = _C>,
        {
            type Output = #target<_C>;

            fn #call(self, rhs: &'a mut #target<_B>) -> Self::Output {
                #target(::core::ops::#op::#call(self.0, &mut rhs.0))
            }
        }

        impl<'a, _A, _B, _C> ::core::ops::#op<&'a mut #target<_B>> for &'a mut #target<_A>
        where
            &'a mut _A: ::core::ops::#op<&'a mut _B, Output = _C>,
        {
            type Output = #target<_C>;

            fn #call(self, rhs: &'a mut #target<_B>) -> Self::Output {
                #target(::core::ops::#op::#call(&mut self.0, &mut rhs.0))
            }
        }

        impl<'a, _A, _B, _C> ::core::ops::#op<#target<_B>> for &'a mut #target<_A>
        where
            &'a mut _A: ::core::ops::#op<_B, Output = _C>,
        {
            type Output = #target<_C>;

            fn #call(self, rhs: #target<_B>) -> Self::Output {
                #target(::core::ops::#op::#call(&mut self.0, rhs.0))
            }
        }
    }
}

fn impl_named(op: &Ident, target: &Ident, call: &Ident, field: &Ident) -> TokenStream {
    quote! {
        impl<_A, _B, _C> ::core::ops::#op<#target<_B>> for #target<_A>
        where
            _A: ::core::ops::#op<_B, Output = _C>,
        {
            type Output = #target<_C>;

            fn #call(self, rhs: #target<_B>) -> Self::Output {
                let #field = ::core::ops::#op::#call(self.#field, rhs.#field);
                #target { #field }
            }
        }

        impl<'a, _A, _B, _C> ::core::ops::#op<&'a #target<_B>> for #target<_A>
        where
            _A: ::core::ops::#op<&'a _B, Output = _C>,
        {
            type Output = #target<_C>;

            fn #call(self, rhs: &'a #target<_B>) -> Self::Output {
                let #field = ::core::ops::#op::#call(self.#field, &rhs.#field);
                #target { #field }
            }
        }

        impl<'a, _A, _B, _C> ::core::ops::#op<&'a #target<_B>> for &'a #target<_A>
        where
            &'a _A: ::core::ops::#op<&'a _B, Output = _C>,
        {
            type Output = #target<_C>;

            fn #call(self, rhs: &'a #target<_B>) -> Self::Output {
                let #field = ::core::ops::#op::#call(&self.#field, &rhs.#field);
                #target { #field }
            }
        }

        impl<'a, _A, _B, _C> ::core::ops::#op<#target<_B>> for &'a #target<_A>
        where
            &'a _A: ::core::ops::#op<_B, Output = _C>,
        {
            type Output = #target<_C>;

            fn #call(self, rhs: #target<_B>) -> Self::Output {
                let #field = ::core::ops::#op::#call(&self.#field, rhs.#field);
                #target { #field }
            }
        }



        impl<'a, _A, _B, _C> ::core::ops::#op<&'a mut #target<_B>> for #target<_A>
        where
            _A: ::core::ops::#op<&'a mut _B, Output = _C>,
        {
            type Output = #target<_C>;

            fn #call(self, rhs: &'a mut #target<_B>) -> Self::Output {
                let #field = ::core::ops::#op::#call(self.#field, &mut rhs.#field);
                #target { #field }
            }
        }

        impl<'a, _A, _B, _C> ::core::ops::#op<&'a mut #target<_B>> for &'a mut #target<_A>
        where
            &'a mut _A: ::core::ops::#op<&'a mut _B, Output = _C>,
        {
            type Output = #target<_C>;

            fn #call(self, rhs: &'a mut #target<_B>) -> Self::Output {
                let #field = ::core::ops::#op::#call(&mut self.#field, &mut rhs.#field);
                #target { #field }
            }
        }

        impl<'a, _A, _B, _C> ::core::ops::#op<#target<_B>> for &'a mut #target<_A>
        where
            &'a mut _A: ::core::ops::#op<_B, Output = _C>,
        {
            type Output = #target<_C>;

            fn #call(self, rhs: #target<_B>) -> Self::Output {
                let #field = ::core::ops::#op::#call(&mut self.#field, rhs.#field);
                #target { #field }
            }
        }
    }
}
