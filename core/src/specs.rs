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
use std::ops::{Index, IndexMut};
use std::vec;

/// [Iterable] describes the basic behaviors of an iterable structure
pub trait Iterable<Idx, T>
where
    Self: Extend<T>
        + FromIterator<T>
        + Index<Idx, Output = T>
        + Insert<Idx, T>
        + IntoIterator<Item = T>,
{
}

/// [ArrayLike] describes the basic behaviors of an array-like structure
pub trait ArrayLike<T: Clone + PartialEq + PartialOrd>:
    AsMut<Vec<T>> + AsRef<Vec<T>> + Eq + IndexMut<usize, Output = T> + Iterable<usize, T> + Ord
{
    /// [ArrayLike::append] describes a method for appending another array to the end of the array
    fn append(&mut self, elem: &mut Self) {
        self.as_mut().append(elem.as_mut());
    }
    fn as_slice(&self) -> &[T] {
        self.as_ref().as_slice()
    }
    /// The capacity of the array
    fn capacity(&self) -> usize {
        self.as_ref().capacity()
    }
    /// [ArrayLike::clear] describes a method for clearing the array
    fn clear(&mut self) {
        self.as_mut().clear();
    }
    /// [ArrayLike::contains] describes a method for checking if an element is present in the array
    fn contains(&self, elem: &T) -> bool {
        self.as_ref().contains(elem)
    }
    /// [ArrayLike::count] describes a method for counting the number of times an element appears in the array
    fn count(&self, elem: &T) -> usize {
        self.as_ref().iter().filter(|&x| x == elem).count()
    }
    /// [ArrayLike::dedup] describes a method for removing duplicate elements from the array
    fn dedup(&mut self) {
        self.as_mut().dedup();
    }
    /// [ArrayLike::dedup_by] describes a method for removing duplicate elements from the array using a custom comparison function
    fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        self.as_mut().dedup_by(same_bucket);
    }
    /// [ArrayLike::dedup_by_key] describes a method for removing duplicate elements from the array using a custom key extraction function
    fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq<K>,
    {
        self.as_mut().dedup_by_key(key);
    }
    /// [ArrayLike::drain] describes a method for removing a range of elements from the array
    fn drain(&mut self, range: std::ops::Range<usize>) -> vec::Drain<T> {
        self.as_mut().drain(range)
    }
    /// [ArrayLike::filter] describes a method for filtering the array
    fn filter(&self, predicate: impl Fn(&T) -> bool) -> Vec<T> {
        self.as_ref()
            .iter()
            .filter(|&x| predicate(x))
            .cloned()
            .collect()
    }
    /// [ArrayLike::first] describes a method for getting a reference to the first element in the array
    fn first(&self) -> Option<&T> {
        self.as_ref().first()
    }
    /// [ArrayLike::get] describes a method for getting a reference to an element at a specific position
    fn get(&self, index: usize) -> Option<&T> {
        if index < self.len() {
            Some(&self[index])
        } else {
            None
        }
    }
    /// [ArrayLike::get_mut] describes a method for getting a mutable reference to an element at a specific position
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len() {
            Some(&mut self[index])
        } else {
            None
        }
    }
    /// [ArrayLike::is_empty] checks if the array is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// [ArrayLike::last] describes a method for gettings the last element in the array
    fn last(&self) -> Option<&T> {
        self.as_ref().last()
    }
    /// [ArrayLike::len] describes a method for getting the length of the array
    fn len(&self) -> usize {
        self.as_ref().len()
    }
    /// [ArrayLike::pop] describes a method for removing the last element from the array
    fn pop(&mut self) -> Option<T> {
        self.as_mut().pop()
    }
    /// [ArrayLike::push] describes a method for adding an element to the end of the array
    fn push(&mut self, elem: T) {
        self.as_mut().push(elem);
    }
    /// [ArrayLike::remove] describes a method for removing an element at a specific position
    fn remove(&mut self, index: usize) -> T {
        self.as_mut().remove(index)
    }
    fn reverse(&mut self) {
        self.as_mut().reverse();
    }
    /// [ArrayLike::set] describes a method for setting the value of an element at a specific position
    fn set(&mut self, index: usize, elem: T) {
        self[index] = elem;
    }
    /// [ArrayLike::shrink_to] describes a method for shrinking the capacity of the array to a specific minimum
    fn shrink_to(&mut self, min_capacity: usize) {
        self.as_mut().shrink_to(min_capacity);
    }
    /// [ArrayLike::shrink_to_fit] describes a method for shrinking the capacity of the array to match its length
    fn shrink_to_fit(&mut self) {
        self.as_mut().shrink_to_fit();
    }
    /// [ArrayLike::splice] describes a method for removing a range of elements and replacing them with another array
    fn splice(&mut self, range: std::ops::Range<usize>, replace_with: Vec<T>) -> Vec<T> {
        self.as_mut().splice(range, replace_with).collect()
    }
    /// [ArrayLike::split_off] describes a method for splitting the array into two at a specific position
    fn split_off(&mut self, at: usize) -> Vec<T> {
        self.as_mut().split_off(at)
    }
    /// [ArrayLike::swap_remove] describes a method for removing an element at a specific position and returning it, replacing it with the last element
    fn swap_remove(&mut self, index: usize) -> T {
        self.as_mut().swap_remove(index)
    }
    /// [ArrayLike::truncate] describes a method for truncating the array to a specific length
    fn truncate(&mut self, len: usize) {
        self.as_mut().truncate(len);
    }
}

/// [Include] describes the basic behaviors of a structure which can include a new element
pub trait Include<T> {
    fn include(&mut self, elem: T);
}

pub trait TryInclude<T> {
    type Error;

    fn try_include<Output>(&mut self, elem: T) -> Result<Output, Self::Error>;
}

/// [Insert] describes the basic behaviors of a structure insert a new element given an index or key
pub trait Insert<Idx, V> {
    fn insert(&mut self, key: Idx, elem: V);
}

pub trait TryInsert<Idx, V> {
    type Error;

    fn try_insert<Output>(&mut self, key: Idx, elem: V) -> Result<Output, Self::Error>;
}

/// [Scope] describes the focus of the [crate::turing::Turing]
pub trait Scope<S: Symbolic>: Include<S> + Insert<usize, S> + Stateful<State> {
    /// [Scope::index] returns the current position of the [Scope] on the [Tape]
    fn index(&self) -> usize;
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
                self.include(elem);
            }
            Move::Left => {
                self.set_index(index - 1);
            }
            Move::Right => {
                self.set_index(index + 1);

                if self.index() == self.tape().len() {
                    self.include(elem);
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

/// [Translate] is a trait that allows for the translation of a machine's memory
pub trait Translate<S: Symbolic> {
    type Error;

    fn translate(&mut self, tape: Tape<S>) -> Result<Tape<S>, Self::Error>;
}

/// [With] describes a simple means of concating several objects together
pub trait With<T> {
    /// [With::Output] must be a superposition of self and T
    type Output;

    /// [With::with] accepts an owned instance of the given type and returns a [With::Output] instance
    fn with(&self, other: &T) -> Self::Output;
}
