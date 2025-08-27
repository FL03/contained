/*
    appellation: macros <test>
    authors: @FL03
*/
use contained::fmt_wrapper;

pub struct A<T>(pub T);

pub struct B<T> {
    pub field: T,
}

fmt_wrapper! {
    A<Q>::(Binary, Debug, Display, LowerHex, UpperHex, LowerExp, UpperExp, Pointer)
}

fmt_wrapper! {
    B<Q>.field::(Binary, Debug, Display, LowerHex, UpperHex, LowerExp, UpperExp, Pointer)
}

#[test]
fn test_fmt_wrapper() {
    let a = A(42);
    let b = B { field: 42 };
    assert_eq!(format!("{}", a), format!("{}", b));
}
