/*
    Appellation: scope <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::{
    states::{State, Stateful},
    turing::{instructions::Move, Tape},
    Symbolic,
};

/// [Scope] describes the focus of the [crate::turing::Turing]
pub trait Scope<S: Symbolic>: Stateful<State = State> {
    /// [Scope::index] returns the current position of the [Scope] on the [Tape]
    fn index(&self) -> usize;
    /// [Scope::insert_symbol] inserts a new element at the current position of the [Scope]
    fn insert_symbol(&mut self, elem: S);
    /// [Scope::set_index] sets the current position of the [Scope] on the [Tape]
    fn set_index(&mut self, pos: usize);
    /// [Scope::set_symbol] sets the current element of the [Scope] on the [Tape]
    fn set_symbol(&mut self, elem: S);
    /// [Move::Left] inserts a new element at the start of the tape if the current position is 0
    /// [Move::Right] inserts a new element at the end of the tape if the current position equals the total number of cells
    /// [Move::Stay] does nothing
    fn shift(&mut self, shift: Move, elem: S) {
        let index = self.index();

        match shift {
            // If the current position is 0, insert a new element at the top of the vector
            Move::Left if self.index() == 0 => {
                self.insert_symbol(elem);
            }
            Move::Left => {
                self.set_index(index - 1);
            }
            Move::Right => {
                self.set_index(index + 1);

                if self.index() == self.tape().len() {
                    self.insert_symbol(elem);
                }
            }
            Move::Stay => {}
        }
    }
    /// [Scope::symbol] returns the current element of the [Scope] on the [Tape]
    fn symbol(&self) -> &S {
        self.tape()
            .get(self.index())
            .expect("Index is out of bounds...")
    }
    /// [Scope::tape] returns the [Tape] of the [Scope]
    fn tape(&self) -> &Tape<S>;
}
