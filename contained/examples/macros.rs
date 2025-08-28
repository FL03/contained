/*
    appellation: macros <example>
    authors: @FL03
*/
use contained::fmt_wrapper;

fn main() -> contained::Result<()> {
    let a = Sample { value: 42 };
    println!("a = {}", a);
    Ok(())
}

pub struct Sample<T> {
    value: T,
}

fmt_wrapper! {
    impl Sample<T>.value {
        Display,
        Debug,
        LowerHex,
        UpperHex,
        LowerExp,
        UpperExp,
        Octal,
        Binary
    }
}
