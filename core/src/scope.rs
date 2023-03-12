/*
    Appellation: scope <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{
    states::{State, States},
    turing::{Move, Tail, Tape, Tapes},
    Symbolic,
};
use std::cell::RefCell;

/// [Scope] describes the focus of the [crate::turing::Turing]
pub trait Scope<S: Symbolic> {
    fn new(index: RefCell<usize>, state: State<States>, tape: Tape<S>) -> Self;
    fn build(tape: Tapes<S>) -> Self
    where
        Self: Sized,
    {
        match tape {
            Tapes::Normal(t) => Self::new(RefCell::new(0), Default::default(), t),
            Tapes::Standard(t) => Self::new(RefCell::new(t.len() - 1), Default::default(), t),
        }
    }
    fn insert(&mut self, elem: S);
    fn position(&self) -> &RefCell<usize>;
    fn set_symbol(&mut self, elem: S);
    /// [Move::Left] inserts a new element at the start of the tape if the current position is 0
    /// [Move::Right] inserts a new element at the end of the tape if the current position equals the total number of cells
    /// [Move::Stay] does nothing
    fn shift(&mut self, shift: Move, elem: S) {
        let index = *self.position().borrow();

        match shift {
            // If the current position is 0, insert a new element at the top of the vector
            Move::Left if *self.position().borrow() == 0 => {
                self.insert(elem);
            }
            Move::Left => {
                self.position().replace(index - 1);
            }
            Move::Right => {
                self.position().replace(index + 1);

                if *self.position().borrow() == self.tape().len() {
                    self.insert(elem);
                }
            }
            Move::Stay => {}
        }
    }
    fn state(&self) -> &State<States>;
    fn scope(&self) -> &S {
        self.tape()
            .get(*self.position().borrow())
            .expect("Index is out of bounds...")
    }
    fn tape(&self) -> &Tape<S>;
    fn update(&mut self, state: Option<State<States>>, symbol: Option<S>);
}
