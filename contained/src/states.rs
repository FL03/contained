/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use decanter::prelude::{hasher, Hashable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct State(usize);

impl State {
    pub fn new(state: usize) -> Self {
        Self(state)
    }
    pub fn state(&self) -> usize {
        self.0
    }
    pub fn update_state(&mut self, state: usize) {
        self.0 = state;
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Hashable for State {
    fn hash(&self) -> decanter::prelude::H256 {
        hasher(&self).into()
    }
}

impl From<usize> for State {
    fn from(data: usize) -> State {
        Self(data)
    }
}

impl From<State> for usize {
    fn from(data: State) -> usize {
        data.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state() {
        let a = State::default();
        let mut b = a.clone();

        assert_eq!(&a, &b);
        assert_eq!(a.state(), 1);

        b.update_state(10);
        assert_eq!(b.state(), 10)
    }
}
