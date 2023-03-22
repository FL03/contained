/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{
    states::{State, Stateful},
    turing::{instructions::Move, Tape},
    Symbolic,
};

/// [ArrayLike] describes the basic behaviors of array-like structures
pub trait ArrayLike<T>: Clone + IntoIterator<Item = T, IntoIter = std::vec::IntoIter<T>> {
    /// [ArrayLike::content]
    fn content(&self) -> &Vec<T>;
    /// [ArrayLike::mut_content]
    fn mut_content(&mut self) -> &mut Vec<T>;
    /// [ArrayLike::append] describes a method which takes another similar array and adds the values to the end of the current array
    fn append(&mut self, elem: &mut Vec<T>) {
        self.mut_content().append(elem);
    }
    /// [ArrayLike::extend] describes a method for extending the array with values from another, similar array
    fn extend(&mut self, elem: impl IntoIterator<Item = T>) {
        self.mut_content().extend(Vec::from_iter(elem));
    }
    /// [ArrayLike::insert] describes a method for inserting a new element at a specific position
    fn insert(&mut self, index: usize, elem: T) {
        self.mut_content().insert(index, elem)
    }
    /// [ArrayLike::is_empty] determines if the array is empty or not
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// [ArrayLike::len] determine the length of the content
    fn len(&self) -> usize {
        self.clone().into_iter().count()
    }
}

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


/// [With] describes a simple means of concating several objects together
pub trait With<T> {
    /// [With::Output] must be a superposition of self and T
    type Output;

    /// [With::with] accepts an owned instance of the given type and returns a [With::Output] instance
    fn with(&self, other: &T) -> Self::Output;
}
