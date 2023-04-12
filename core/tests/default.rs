#[cfg(test)]
#[test]
fn compiles() {
    let f = |x: usize| x + 1;
    assert_eq!(f(1), 2);
}
