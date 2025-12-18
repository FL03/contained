/*
    Appellation: ast <module>
    Created At: 2025.12.18:07:44:04
    Contrib: @FL03
*/
#![allow(dead_code)]

mod ops;
mod wrapper;

use syn::token::Impl;
use syn::{AngleBracketedGenericArguments, Ident, Token, WhereClause};

/// The abstract syntax tree for the `binary_wrapper` macro input;
/// e.g. `impl A { Add.add, Sub.sub }` or `impl B.field { Add.add, Sub.sub }`
pub struct WrapperOpsAst {
    pub _impl: Impl,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub target: Ident,
    pub field: Option<Ident>,
    pub where_clause: Option<WhereClause>,
    pub ops: Vec<(Ident, Ident)>,
}

pub struct WrapperAst {
    pub target: Ident,
    pub field: Option<NamedFieldAst>,
}

pub struct NamedFieldAst {
    pub period: Token![.],
    pub field: Ident,
}
