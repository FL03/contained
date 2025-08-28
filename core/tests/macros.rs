/*
    appellation: macros <test>
    authors: @FL03
*/
use contained_core::impl_wrapper_unary;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct A<T>(pub T);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct B<T> {
    pub field: T,
}

impl_wrapper_unary! {
    A {
        Not.not
    }
}

impl_wrapper_unary! {
    B.field {
        Not.not
    }
}

#[test]
fn test_unary_impls() {
    let a = A(true);
    let b = B { field: true };
    assert_eq!(!a, A(false));
    assert_eq!(!b, B { field: false });
}
