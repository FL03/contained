#[cfg(test)]
#[test]
fn compiles() {
    let f = |i: usize| i * i;

    assert_eq!(f(2), 4)
}
