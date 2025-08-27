/*
    appellation: macros <test>
    authors: @FL03
*/
use contained_core::impl_wrapper_binary;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct A<T>(pub T);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct B<T> {
    pub field: T,
}

impl_wrapper_binary! {
    A {
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

// impl_wrapper_binary! {
//     B.field {
//         Add.add,
//         Sub.sub,
//         Mul.mul,
//         Div.div,
//         Rem.rem,
//         BitAnd.bitand,
//         BitOr.bitor,
//         BitXor.bitxor,
//         Shl.shl,
//         Shr.shr,
//     }
// }

#[test]
fn test_binary_ops() {
    let (x, y) = (A(42), A(2));
    assert_eq!((x + y), A(44));
    assert_eq!((x - y), A(40));
}
