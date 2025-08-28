/*
    appellation: macros <test>
    authors: @FL03
*/
use contained_macros::binary_wrapper;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct A<T>(pub T);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct B<T> {
    pub value: T,
}

binary_wrapper! {
    impl A {
        Add.add,
        Sub.sub,
        Mul.mul,
        Div.div,
        Rem.rem,
        BitAnd.bitand,
        BitOr.bitor,
        BitXor.bitxor,
        Shl.shl,
        Shr.shr,
    }
}

binary_wrapper! {
    impl B.value {
        Add.add,
        Sub.sub,
        Mul.mul,
        Div.div,
        Rem.rem,
        BitAnd.bitand,
        BitOr.bitor,
        BitXor.bitxor,
        Shl.shl,
        Shr.shr,
    }
}

#[test]
fn test_impl_binary_ops_on_tuple() {
    let (x, y) = (A(42), A(&2));
    assert_eq!((x + y), A(44));
    assert_eq!((x - y), A(40));
}

#[test]
fn test_impl_binary_ops_on_struct() {
    let (x, y) = (B { value: 42 }, B { value: 2 });
    assert_eq!((x + y), B { value: 44 });
    assert_eq!((x - y), B { value: 40 });
}
