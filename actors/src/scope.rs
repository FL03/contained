/*
    Appellation: actor <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{
    states::{State, States},
    turing::{Move, Tape, Tapes},
    Symbolic,
};

/// [Scope] describes the focus of the [crate::turing::Turing]
pub trait Scope<S: Symbolic> {
    fn new(index: usize, state: State<States>, tape: Tape<S>) -> Self;
    fn build(tape: Tapes<S>) -> Self
    where
        Self: Sized,
    {
        match tape {
            Tapes::Normal(t) => Self::new(0, Default::default(), t),
            Tapes::Standard(t) => Self::new(t.len() - 1, Default::default(), t),
        }
    }
    fn insert(&mut self, elem: S);
    fn position(&self) -> usize;
    fn set_position(&mut self, index: usize);
    fn set_state(&mut self, state: State<States>);
    fn set_symbol(&mut self, elem: S);
    /// [Move::Left] inserts a new element at the start of the tape if the current position is 0
    /// [Move::Right] inserts a new element at the end of the tape if the current position equals the total number of cells
    /// [Move::Stay] does nothing
    fn shift(&mut self, shift: Move, elem: S) {
        let index = self.position();
        match shift {
            // If the current position is 0, insert a new element at the top of the vector
            Move::Left if self.position() == 0 => {
                self.insert(elem);
            }
            Move::Left => {
                self.set_position(index - 1);
            }
            Move::Right => {
                self.set_position(index + 1);

                if self.position() == self.tape().len() {
                    self.insert(elem);
                }
            }
            Move::Stay => {}
        }
    }
    fn state(&self) -> &State<States>;
    fn scope(&self) -> &S {
        self.tape()
            .get(self.position())
            .expect("Index is out of bounds...")
    }
    fn tape(&self) -> &Tape<S>;
}
