/*
    appellation: wrapper <module>
    authors: @FL03
*/
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Field, Generics, Ident};

pub fn impl_get(
    DeriveInput {
        data,
        generics,
        ident: name,
        ..
    }: &DeriveInput,
) -> proc_macro2::TokenStream {
    // split the generics for implementation
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    // handle the case where the data is a struct
    match data {
        Data::Struct(DataStruct { fields, .. }) => {
            // handle the fields
            let methods = fields
                .iter()
                .map(|field| _handle_field(field, generics, name));
            // inject generics to ensure the wrapper can be used with generic types
            return quote! {
                impl #impl_generics #name #ty_generics #where_clause {
                    #(#methods)*
                }
            };
        }
        _ => panic!("The `Get` macro can only be derived for structs"),
    }
}

fn _handle_field(
    field: &Field,
    generics: &Generics,
    name: &syn::Ident,
) -> proc_macro2::TokenStream {
    // handle both named and unnamed fields
    let methods = match field.ident {
        Some(_) => handle_get_named(field, generics, name),
        None => handle_get_unnamed(field, generics, name),
    };
    // generate the code for the wrapper methods
    quote! {
        #methods
    }
}

fn handle_get_named(
    field: &Field,
    generics: &Generics,
    _name: &syn::Ident,
) -> proc_macro2::TokenStream {
    let Field {
        ident,
        ty: field_type,
        ..
    } = field;

    let _where_clause_u = generics.where_clause.as_ref();
    // get a reference to the field name
    let field_name = ident.as_ref().unwrap();
    let get = field_name;
    let get_mut = quote::format_ident!("{}_mut", field_name);
    let into = quote::format_ident!("into_{}", field_name);
    // implement the methods for named fields
    quote! {
        /// returns a reference to the value
        pub const fn #get(&self) -> &#field_type {
            &self.#field_name
        }
        /// returns a mutable reference to the value
        pub const fn #get_mut(&mut self) -> &mut #field_type {
            &mut self.#field_name
        }
        #[inline]
        /// consumes the current instance to return the inner value
        pub fn #into(self) -> #field_type {
            self.#field_name
        }
    }
}

fn handle_get_unnamed(
    field: &Field,
    _generics: &Generics,
    _name: &syn::Ident,
) -> proc_macro2::TokenStream {
    let field_type = &field.ty;
    quote! {
        /// returns a reference to the wrapped field
        pub const fn get(&self) -> &#field_type {
            &self.0
        }
        /// returns a mutable reference to the wrapped field
        pub const fn get_mut(&mut self) -> &mut #field_type {
            &mut self.0
        }
        /// consumes the current instance and returns the wrapped field
        #[inline]
        pub fn value(self) -> #field_type {
            self.0
        }
    }
}

fn _convert_generic_where_clause(
    new_ident: &Ident,
    clause: &syn::WhereClause,
) -> proc_macro2::TokenStream {
    let predicates = clause.predicates.iter().map(|p| {
        if let syn::WherePredicate::Type(inner) = p {
            let mut pred = inner.clone();
            pred.bounded_ty = if let syn::Type::Verbatim(_ty) = &inner.bounded_ty {
                syn::Type::Verbatim(quote!(#new_ident))
            } else {
                inner.bounded_ty.clone()
            };
            // For other types of predicates, we can just return them as is
            return quote!(#pred);
        }
        // For other types of predicates, we can just return them as is
        quote!(#p)
    });
    quote! {
        where #(#predicates),*
    }
}
