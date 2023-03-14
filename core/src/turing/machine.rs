/*
    Appellation: machine <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{
    instructions::Head,
    tapes::{Tape, Tapes},
    Operator, Program, Turing,
};
use crate::states::{Stateful, States};
use crate::{Scope, Symbolic};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Machine<S: Symbolic = String> {
    driver: Operator<S>,
    program: Program<S>,
}

impl<S: Symbolic> Machine<S> {
    pub fn new(driver: Operator<S>, program: Program<S>) -> Self {
        Self { driver, program }
    }
    pub fn driver(&self) -> &Operator<S> {
        &self.driver
    }
    pub fn program(&self) -> &Program<S> {
        &self.program
    }
    pub fn tape(&self) -> &Tape<S> {
        self.driver.tape()
    }
}

impl<S: Symbolic> Turing<S> for Machine<S> {
    type Error = Box<dyn std::error::Error>;
    type Scope = Operator<S>;

    fn execute(&mut self) -> Result<&Self, Self::Error> {
        let until = |actor: &Operator<S>| actor.state().state() == States::Invalid;
        self.execute_until(until)
    }

    fn execute_once(&mut self) -> Result<&Self, Self::Error> {
        let head = Head::new(self.driver().state().clone(), self.driver.scope().clone());
        let inst = self.program.get(head)?.clone();
        self.driver.update(
            Some(inst.tail().state().clone()),
            Some(inst.tail().symbol()),
        );
        self.driver
            .shift(inst.tail().action(), self.program.default_symbol().clone());
        Ok(self)
    }

    fn execute_until(
        &mut self,
        until: impl Fn(&Self::Scope) -> bool,
    ) -> Result<&Self, Self::Error> {
        while !until(&self.driver) {
            let head = Head::new(self.driver.state().clone(), self.driver.scope().clone());
            let inst = self.program.get(head)?.clone();
            self.driver.update(
                Some(inst.tail().state().clone()),
                Some(inst.tail().symbol()),
            );
            self.driver
                .shift(inst.tail().action(), self.program.default_symbol().clone());
        }
        Ok(self)
    }

    fn translate(&mut self, tape: Tapes<S>) -> Result<Tape<S>, Self::Error> {
        self.driver = Operator::build(tape);
        self.execute()?;
        Ok(self.driver.tape().clone())
    }
}
