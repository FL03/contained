/*
    appellation: macros <test>
    authors: @FL03
*/
use contained_core::fmt_wrapper;

#[test]
fn test_fmt_wrapper() {
    pub struct A<T>(pub T);

    pub struct B<T> {
        pub field: T,
    }

    fmt_wrapper! {
        A<Q>::(Binary, Debug, Display, LowerHex, UpperHex, LowerExp, UpperExp, Pointer)
    };
    fmt_wrapper! {
        B<Q>.field::(Binary, Debug, Display, LowerHex, UpperHex, LowerExp, UpperExp, Pointer)
    }
    let sample = A(42);
    let sample2 = B { field: 42 };
    assert_eq!(format!("{}", sample), format!("{}", sample2));
}
