#[cfg(test)]
use contained_music::{absmod, intervals, Note};

#[test]
fn compiles() {
    let f = |i: usize| i * i;

    assert_eq!(f(2), 4)
}

#[test]
fn test_absmod() {
    let a: i64 = -13 % 12;
    assert_ne!(a.abs(), absmod(-13, 12));
    assert_eq!(absmod(-1, 12), 11);
}

#[test]
fn test_intervals() {
    let notes: Vec<Note> = vec![0.into(), 4.into(), 7.into()];
    assert_eq!(
        intervals(notes),
        vec![
            ((Note::from(0), Note::from(4)), 4),
            ((Note::from(4), Note::from(7)), 3),
            ((Note::from(7), Note::from(0)), -7),
        ]
    );
}
