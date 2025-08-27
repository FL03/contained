
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident, Token, braced, parse::{Parse, ParseStream}};

/// Input syntax: WrapperType, field, [ (OpTrait, fn_name), ... ]
struct WrapperOpsInput {
    wrapper: Ident,
    field: Ident,
    ops: Vec<(Ident, Ident)>,
}

impl Parse for WrapperOpsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let wrapper: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let field: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let content;
        braced!(content in input);
        let mut ops = Vec::new();
        while !content.is_empty() {
            let op: Ident = content.parse()?;
            content.parse::<Token![.]>()?;
            let call: Ident = content.parse()?;
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
            ops.push((op, call));
        }
        Ok(Self { wrapper, field, ops })
    }
}

/// Procedural macro entry point
pub fn wrapper_ops_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let WrapperOpsInput { wrapper, field, ops } = parse_macro_input!(input as WrapperOpsInput);

    let mut impls = Vec::new();
    for (op, call) in ops {
        let op_assign = format_ident!("{}Assign", op);
        let call_assign = format_ident!("{}_assign", call);

        impls.push(quote! {
            impl<A, B, C> ::core::ops::#op<#wrapper<B>> for #wrapper<A>
            where
                A: ::core::ops::#op<B, Output = C>,
            {
                type Output = #wrapper<C>;
                fn #call(self, rhs: #wrapper<B>) -> Self::Output {
                    #wrapper(::core::ops::#op::#call(self.#field, rhs.#field))
                }
            }

            impl<A, B> ::core::ops::#op_assign<#wrapper<B>> for #wrapper<A>
            where
                A: ::core::ops::#op_assign<B>,
            {
                fn #call_assign(&mut self, rhs: #wrapper<B>) {
                    ::core::ops::#op_assign::#call_assign(&mut self.#field, rhs.#field)
                }
            }
        });
    }

    TokenStream::from(quote! { #(#impls)* }).into()
}