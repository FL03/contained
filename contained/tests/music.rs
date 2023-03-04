#[cfg(test)]
use contained::Gradient;

#[test]
fn test_gradient() {
    let b = -13;
    assert_eq!(144_i64.pitch(), 0);
    assert_eq!(b.pitch(), 11)
}
