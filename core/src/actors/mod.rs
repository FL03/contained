/*
    Appellation: actors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::actor::*;

mod actor;
pub mod exec;

use crate::states::{State, Stateful};
use crate::turing::instructions::Instruction;
use crate::{Alphabet, Error, Scope, Symbolic};
use async_trait::async_trait;
use futures::{Future, StreamExt};
use predicates::Predicate;

pub trait Executable<S: Symbolic>: Clone + Alphabet<S> + Iterator<Item = Instruction<S>> {
    type Driver: Scope<S>;
    type Error;

    fn execute(&mut self, driver: &mut Self::Driver) -> Result<Self::Driver, Self::Error> {
        // Get the default symbol
        let default_symbol = self.clone().default_symbol();
        // Get the next instruction
        while let Some(instruction) = self.next() {
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
    fn execute_until(
        &mut self,
        driver: &mut Self::Driver,
        until: impl Fn(&Self::Driver) -> bool,
    ) -> Result<Self::Driver, Self::Error> {
        // Get the default symbol
        let default_symbol = self.clone().default_symbol();
        // Get the next instruction
        while let Some(instruction) = self.next() {
            let tail = instruction.clone().tail();
            // Update the current state
            driver.update_state(tail.state());
            // Update the tape
            driver.set_symbol(tail.symbol());
            // Update the index; adjusts the index according to the direction
            driver.shift(tail.action(), default_symbol.clone());
            if until(driver) {
                break;
            }
        }
        // Return the actor
        Ok(driver.clone())
    }
}

#[async_trait]
pub trait AsyncExecute<S: Symbolic + Send + Sync>:
    Alphabet<S> + StreamExt<Item = Instruction<S>> + Stateful<State> + Unpin
{
    type Driver: Future + Scope<S> + Send + Sync;
    type Error: Send + Sync;

    async fn execute(&mut self) -> Result<Self::Driver, Self::Error> {
        // Get the default symbol
        let default_symbol = self.clone().default_symbol();
        // Get the next instruction
        while let Some(instruction) = self.next().await {
            // Get the tail of the instruction
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
    /// Returns a reference to the scope
    fn scope(&self) -> Self::Driver;
    /// Returns a mutable reference to the scope
    fn scope_mut(&mut self) -> &mut Self::Driver;
}

/// [Execute] is a trait that allows for the execution of a program.
pub trait Execute<S: Symbolic>:
    Alphabet<S> + Iterator<Item = Instruction<S>> + Stateful<State>
{
    type Driver: Scope<S>;

    /// [Execute::execute]
    fn execute(&mut self) -> Result<Self::Driver, Error> {
        // Get the default symbol
        let default_symbol = self.clone().default_symbol();
        // Get the next instruction
        while let Some(instruction) = self.next() {
            let tail = instruction.clone().tail();
            // Update the current state
            self.scope_mut().update_state(tail.state());
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
    fn execute_once(&mut self) -> Result<Self::Driver, Error> {
        // Get the default symbol
        let default_symbol = self.clone().default_symbol();
        // Get the next instruction
        if let Some(instruction) = self.next() {
            let tail = instruction.clone().tail();
            // Update the current state
            self.scope_mut().update_state(tail.state());
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
        until: &dyn Predicate<Self::Driver>,
    ) -> Result<Self::Driver, Error> {
        while !until.eval(&self.scope()) {
            self.execute_once()?;
        }
        Ok(self.scope())
    }

    fn scope(&self) -> Self::Driver;

    fn scope_mut(&mut self) -> &mut Self::Driver;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::states::State;
    use crate::turing::{
        instructions::{Instruction, Move},
        Program, Tape,
    };

    pub const TEST_ALPHABET: [&str; 3] = ["a", "b", "c"];

    #[test]
    fn test_actor() {
        // Instruction set; turn ["a", "b", "c"] into ["c", "a", "a"]
        let instructions: Vec<Instruction<&str>> = vec![
            (State::default(), "a", State::default(), "c", Move::Right).into(),
            (State::default(), "b", State::default(), "a", Move::Right).into(),
            (State::default(), "c", State::invalid(), "a", Move::Stay).into(),
        ];
        // Setup the program
        let program = Program::new(TEST_ALPHABET, State::invalid());
        // Initialize a new machine
        let mut machine = Actor::new(program, None);
        // Extend the machine memory
        machine.extend(Tape::norm(["a", "b", "c"]));
        // Extend the program; turn [0, 3, 6] into [6, 3, 3]
        machine.extend(instructions);
        // Execute the program
        assert!(machine.execute().is_ok());
        // Assert the result
        assert_eq!(machine.memory.clone(), Tape::norm(["c", "a", "a"]));
    }
}
