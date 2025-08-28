/*
    appellation: derive <example>
    authors: @FL03
*/

fn main() {
    let mut a = A::new(1).map(|x| x + 100);
    assert_eq!(a.get(), &101);
    a.set(202);
    assert_eq!(a.get_mut(), &mut 202);
}

use contained::Wrapper;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Wrapper)]
pub struct A<T>(T);