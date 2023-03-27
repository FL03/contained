/*
    Appellation: machine <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::instructions::Instruction;
use super::{Driver, Program, Tape, Turing};
use crate::{Alphabet, ArrayLike, Error, Scope, State, Stateful, Symbolic, Translate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Machine<S: Symbolic = String> {
    pub driver: Driver<S>,
    pub program: Program<S>,
}

impl<S: Symbolic> Machine<S> {
    pub fn new(driver: Driver<S>, program: Program<S>) -> Self {
        Self { driver, program }
    }
    pub fn program(&self) -> Program<S> {
        self.program.clone()
    }
    pub fn scope(&self) -> Driver<S> {
        self.driver.clone()
    }
    pub fn tape(&self) -> Tape<S> {
        self.driver.tape()
    }
}

impl<S: Symbolic> Extend<S> for Machine<S> {
    fn extend<T: IntoIterator<Item = S>>(&mut self, iter: T) {
        self.driver.memory.extend(iter)
    }
}

impl<S: Symbolic> Extend<Instruction<S>> for Machine<S> {
    fn extend<T: IntoIterator<Item = Instruction<S>>>(&mut self, iter: T) {
        self.program.extend(iter)
    }
}

impl<S: Symbolic> Iterator for Machine<S> {
    type Item = Instruction<S>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.clone().driver.tape().get(self.driver.cursor()) {
            // Get the instruction
            self.program
                .get((self.state(), cur.clone()).into())
                .cloned()
        } else {
            None
        }
    }
}

impl<S: Symbolic> Alphabet<S> for Machine<S> {
    fn is_viable(&self, symbol: &S) -> bool {
        self.program.is_viable(symbol)
    }
    fn default_symbol(&self) -> S {
        self.program.default_symbol()
    }
}

impl<S: Symbolic> Stateful<State> for Machine<S> {
    fn state(&self) -> State {
        self.driver.state()
    }

    fn update_state(&mut self, state: State) {
        self.driver.update_state(state)
    }
}

impl<S: Symbolic> Turing<S> for Machine<S> {
    type Scope = Driver<S>;

    fn execute(&mut self) -> Result<&Self, Self::Error> {
        let until = |actor: &Driver<S>| actor.state() == State::Invalid;
        self.execute_until(until)
    }

    fn execute_once(&mut self) -> Result<&Self, Self::Error> {
        let head = self.driver.clone().into();
        let inst = self.program.get(head).expect("").clone();
        self.driver.update_state(inst.tail().state());
        self.driver.set_symbol(inst.tail().symbol());
        self.driver
            .shift(inst.tail().action(), self.program.default_symbol());
        Ok(self)
    }

    fn execute_until(
        &mut self,
        until: impl Fn(&Self::Scope) -> bool,
    ) -> Result<&Self, Self::Error> {
        while !until(&self.driver) {
            self.execute_once()?;
        }
        Ok(self)
    }
}

impl<S: Symbolic> Translate<S> for Machine<S> {
    type Error = Error;

    fn translate(&mut self, tape: Tape<S>) -> Result<Tape<S>, Self::Error> {
        self.driver = Driver::from(tape);
        self.execute()?;
        Ok(self.driver.tape().clone())
    }
}
