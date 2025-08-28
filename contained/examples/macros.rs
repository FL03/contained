/*
    appellation: macros <example>
    authors: @FL03
*/
use contained::fmt_wrapper;

fn main() -> contained::Result<()> {
    let a = A(255);
    let b = B { value: 255 };
    println!("A: {a:?}, B: {b:?}");
    assert_eq!(format!("{:x}", a), format!("{:x}", b));
    Ok(())
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd)]
pub struct A<T>(T);

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd)]
pub struct B<T> {
    pub value: T,
}

fmt_wrapper! {
    impl A<T> {
        Debug,
        Display,
        LowerHex,
        UpperHex,
        LowerExp,
        UpperExp,
        Binary,
        Octal,
        Pointer,
    }
}

fmt_wrapper! {
    impl B<T>.value {
        Debug,
        Display,
        LowerHex,
        UpperHex,
        LowerExp,
        UpperExp,
        Binary,
        Octal,
        Pointer,
    }
}
