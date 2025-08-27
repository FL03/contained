/*
    appellation: wrapper <module>
    authors: @FL03
*/
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Field, Generics, Ident};

pub fn impl_wrapper(input: &DeriveInput) -> proc_macro2::TokenStream {
    // deconstruct the input to get the struct name and generics
    let DeriveInput {
        data,
        generics,
        ident: name,
        .. // ignore other fields
    } = input;
    // split the generics for implementation
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    // handle the case where the data is a struct
    if let Data::Struct(DataStruct { fields, .. }) = data {
        // ensure the struct is a single field struct
        if fields.len() != 1 {
            panic!("The `Wrapper` macro can only be derived for single field structs");
        }
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

    panic!("The `Wrapper` macro can only be derived for single field structs");
}

fn _handle_field(
    field: &Field,
    generics: &Generics,
    name: &syn::Ident,
) -> proc_macro2::TokenStream {
    // deconstruct the field for easier access
    let Field {
        ident: field_name,
        ty: field_type,
        ..
    } = field;
    // handle both named and unnamed fields
    let methods = match field_name {
        Some(_) => _handle_named(field, generics, name),
        None => _handle_unnamed(field, generics, name),
    };
    // generate the code for the wrapper methods
    quote! {
        #methods
        /// consumes the current instance and returns a new one that captures the result of the
        /// closure on the wrapped field
        #[inline]
        pub fn map<U, F>(self, f: F) -> #name<U>
        where
            F: FnOnce(#field_type) -> U,
        {
            #name::new(f(self.value()))
        }
        /// [`replace`](core::mem::replace) the wrapped field with a new value and return
        /// the old value
        pub const fn replace(&mut self, value: #field_type) -> #field_type {
            ::core::mem::replace(self.get_mut(), value)
        }
        /// set the wrapped field to a new value and return a mutable reference to the
        /// current instance
        #[inline]
        pub fn set(&mut self, value: #field_type) -> &mut Self {
            *self.get_mut() = value;
            self
        }
        /// [`swap`](core::mem::swap) the wrapped field with another instance
        pub const fn swap(&mut self, other: &mut Self) {
            ::core::mem::swap(self.get_mut(), other.get_mut());
        }
        /// [`take`](core::mem::take) the wrapped field and replace it with a default value
        #[inline]
        pub fn take(&mut self) -> #field_type
        where
            #field_type: Default
        {
            ::core::mem::take(self.get_mut())
        }
        /// returns a new instance of the wrapper that contains a reference to the inner value
        pub const fn view(&self) -> #name<&#field_type> {
            #name::new(self.get())
        }
        /// returns a new instance of the wrapper that contains a mutable reference to the
        /// inner value
        pub const fn view_mut(&mut self) -> #name<&mut #field_type> {
            #name::new(self.get_mut())
        }
    }
}

fn _handle_named(
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
    if ident.is_none() {
        panic!("The `Wrapper` macro can only be derived for single field structs");
    }
    // get a reference to the field name
    let field_name = ident.as_ref().unwrap();
    // implement the methods for named fields
    quote! {
        pub const fn new(#field_name: #field_type) -> Self {
            Self { #field_name }
        }
        /// returns a reference to the wrapped field
        pub const fn get(&self) -> &#field_type {
            &self.#field_name
        }
        /// returns a mutable reference to the wrapped field
        pub const fn get_mut(&mut self) -> &mut #field_type {
            &mut self.#field_name
        }
        /// consumes the current instance and returns the wrapped field
        #[inline]
        pub fn value(self) -> #field_type {
            self.#field_name
        }
    }
}

fn _handle_unnamed(
    field: &Field,
    _generics: &Generics,
    _name: &syn::Ident,
) -> proc_macro2::TokenStream {
    let field_type = &field.ty;
    quote! {
        pub const fn new(value: #field_type) -> Self {
            Self(value)
        }
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
