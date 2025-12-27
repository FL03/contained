/*
    Appellation: ast <module>
    Created At: 2025.12.18:07:44:04
    Contrib: @FL03
*/
mod impl_ast_wrapper;

use syn::punctuated::Punctuated;
use syn::token::Impl;
use syn::{AngleBracketedGenericArguments, Ident, Token, WhereClause};

#[allow(dead_code)]
/// The abstract syntax tree for the `binary_wrapper` macro input;
/// e.g. `impl A { Add.add, Sub.sub }` or `impl B.field { Add.add, Sub.sub }`
pub struct WrapperImpls {
    pub impl_token: Impl,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub target: Ident,
    pub field: Option<Ident>,
    pub where_clause: Option<WhereClause>,
    pub ops: Punctuated<MethodCallAst, Token![,]>,
}

#[allow(dead_code)]
pub struct MethodCallAst {
    pub name: Ident,
    pub dot: Token![.],
    pub call: Ident,
}
