/*
    appellation: macros <test>
    authors: @FL03
*/
use contained::fmt_wrapper;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct A<T>(pub T);

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct B<T> {
    pub value: T,
}

fmt_wrapper! {
    impl A<Q> { 
        Binary, 
        Debug, 
        Display, 
        LowerHex, 
        UpperHex, 
        LowerExp, 
        UpperExp, 
        Pointer 
    }
}

fmt_wrapper! {
    impl B<Q>.value { 
        Binary, 
        Debug, 
        Display, 
        LowerHex, 
        UpperHex, 
        LowerExp, 
        UpperExp, 
        Pointer 
    }
}

#[test]
fn test_fmt_wrapper() {
    let a = A(42);
    let b = B { value: 42 };
    assert_eq!(format!("{}", a), format!("{}", b));

    let a = A(3.14);
    assert_eq!(format!("{:.3}", a), format!("{:.3}", 3.14));
}
