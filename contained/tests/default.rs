/*
    appellation: default <module>
    authors: @FL03
*/

fn adder<A, B, C>(a: A, b: B) -> C
where
    A: core::ops::Add<B, Output = C>,
{
    a + b
}

#[test]
fn compiles() {
    assert_eq!(adder(1, 100), 101);
    assert_eq!(adder(1.0, 100.0), 101.0);
}
