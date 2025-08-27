
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident, Token, braced, parse::{Parse, ParseStream}};

pub struct WrapperOpsInput {
    pub target: Ident,
    pub field: Option<Ident>,
    pub ops: Vec<(Ident, Ident)>,
}

impl Parse for WrapperOpsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let target: Ident = input.parse()?;
        let field = if input.peek(Token![.]) {
            input.parse::<Token![.]>()?;
            Some(input.parse()?)
        } else {
            None
        };
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
        Ok(Self { target, field, ops })
    }
}
/// Procedural macro entry point
pub fn wrapper_ops_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let WrapperOpsInput {target, field, ops } = parse_macro_input!(input as WrapperOpsInput);

    let mut impls = Vec::new();
    for (op, call) in ops {
        let op_assign = format_ident!("{}Assign", op);
        let call_assign = format_ident!("{}_assign", call);

        impls.push(quote! {
            impl<A, B, C> ::core::ops::#op<#target<B>> for #target<A>
            where
                A: ::core::ops::#op<B, Output = C>,
            {
                type Output = #target<C>;
                fn #call(self, rhs: #target<B>) -> Self::Output {
                    #target(::core::ops::#op::#call(self.#field, rhs.#field))
                }
            }

            impl<A, B> ::core::ops::#op_assign<#target<B>> for #target<A>
            where
                A: ::core::ops::#op_assign<B>,
            {
                fn #call_assign(&mut self, rhs: #target<B>) {
                    ::core::ops::#op_assign::#call_assign(&mut self.#field, rhs.#field)
                }
            }
        });
    }

    TokenStream::from(quote! { #(#impls)* }).into()
}