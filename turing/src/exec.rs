/*
    Appellation: exec <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::instructions::Instruction;
use crate::{Alphabet, Program, Scope, Symbolic};
use async_trait::async_trait;
use contained_core::{
    states::{State, Stateful},
    Error,
};
use futures::{Future, StreamExt};
use predicates::Predicate;
use std::sync::{Arc, Mutex};

/// [AsyncExecute] describes a self-contained executor that can be executed asynchronously.
#[async_trait]
pub trait AsyncExecute<S: Symbolic + Send + Sync>:
    Alphabet<S> + StreamExt<Item = Instruction<S>> + Stateful<State> + Unpin
{
    type Driver: Future + Scope<S> + Send + Sync;
    type Error: Send + Sync;

    async fn execute(&mut self) -> Result<&Arc<Mutex<Self::Driver>>, Self::Error> {
        // Get the default symbol
        let default_symbol = self.clone().default_symbol();
        // Get the next instruction
        while let Some(instruction) = self.next().await {
            // Get the tail of the instruction
            let tail = instruction.clone().tail();
            // Update the current state
            self.update_state(tail.state());
            // Update the tape
            self.scope_mut().lock().unwrap().set_symbol(tail.symbol());
            // Update the index; adjusts the index according to the direction
            self.scope_mut()
                .lock()
                .unwrap()
                .shift(tail.action(), default_symbol.clone());
        }
        // Return the actor
        Ok(self.scope())
    }
    /// Returns a reference to the scope
    fn scope(&self) -> &Arc<Mutex<Self::Driver>>;
    /// Returns a mutable reference to the scope
    fn scope_mut(&mut self) -> &mut Arc<Mutex<Self::Driver>>;
}

/// [Execute] describes a self-contained executor that can be executed synchronously.
pub trait Execute<S: Symbolic>:
    Alphabet<S> + Iterator<Item = Instruction<S>> + Stateful<State>
{
    type Driver: Scope<S>;

    /// [Execute::execute]
    fn execute(&mut self) -> Result<&Self::Driver, Error> {
        // Get the default symbol
        let default_symbol = self.program().default_symbol();
        // Get the next instruction
        while let Some(instruction) = self.next() {
            let tail = instruction.clone().tail();
            // Update the current state
            self.update_state(tail.state());
            // Update the tape
            self.scope_mut().set_symbol(tail.symbol());
            // Update the index; adjusts the index according to the direction
            self.scope_mut()
                .shift(tail.action(), default_symbol.clone());
        }
        // Return the actor
        Ok(self.scope())
    }
    /// [Execute::execute_once]
    fn execute_once(&mut self) -> Result<&Self::Driver, Error> {
        // Get the default symbol
        let default_symbol = self.clone().default_symbol();
        // Get the next instruction
        if let Some(instruction) = self.next() {
            let tail = instruction.tail();
            // Update the current state
            self.update_state(tail.state());
            // Update the tape
            self.scope_mut().set_symbol(tail.symbol());
            // Update the index; adjusts the index according to the direction
            self.scope_mut().shift(tail.action(), default_symbol);
            // Return the actor
            return Ok(self.scope());
        }
        Err(Error::ExecutionError(
            "No more instructions to execute".into(),
        ))
    }
    /// [Execute::execute_until]
    fn execute_until(
        &mut self,
        until: impl Predicate<Self::Driver>,
    ) -> Result<&Self::Driver, Error> {
        while !until.eval(self.scope()) {
            self.execute_once()?;
        }
        Ok(self.scope())
    }

    fn program(&self) -> &Program<S>;

    fn scope(&self) -> &Self::Driver;

    fn scope_mut(&mut self) -> &mut Self::Driver;
}

/// [Executable] describes a program that can be executed with an external driver.
pub trait Executable<S: Symbolic>: Clone + Alphabet<S> + Iterator<Item = Instruction<S>> {
    type Driver: Scope<S>;
    type Error;

    fn execute(&mut self, driver: &mut Self::Driver) -> Result<Self::Driver, Self::Error> {
        // Get the default symbol
        let default_symbol = self.clone().default_symbol();
        // Get the next instruction
        for instruction in self.by_ref() {
            let tail = instruction.clone().tail();
            // Update the current state
            driver.update_state(tail.state());
            // Update the tape
            driver.set_symbol(tail.symbol());
            // Update the index; adjusts the index according to the direction
            driver.shift(tail.action(), default_symbol.clone());
        }
        // Return the actor
        Ok(driver.clone())
    }
    fn execute_once(&mut self, driver: &mut Self::Driver) -> Result<Self::Driver, Self::Error> {
        // Get the default symbol
        let default_symbol = self.clone().default_symbol();
        // Get the next instruction
        if let Some(instruction) = self.next() {
            let tail = instruction.tail();
            // Update the current state
            driver.update_state(tail.state());
            // Update the tape
            driver.set_symbol(tail.symbol());
            // Update the index; adjusts the index according to the direction
            driver.shift(tail.action(), default_symbol);
        }
        // Return the actor
        Ok(driver.clone())
    }
    fn execute_until(
        &mut self,
        driver: &mut Self::Driver,
        until: impl Predicate<Self::Driver>,
    ) -> Result<Self::Driver, Self::Error> {
        // Get the default symbol
        let default_symbol = self.clone().default_symbol();
        // Get the next instruction
        for instruction in self.by_ref() {
            let tail = instruction.clone().tail();
            // Update the current state
            driver.update_state(tail.state());
            // Update the tape
            driver.set_symbol(tail.symbol());
            // Update the index; adjusts the index according to the direction
            driver.shift(tail.action(), default_symbol.clone());
            if until.eval(driver) {
                break;
            }
        }
        // Return the actor
        Ok(driver.clone())
    }
}
