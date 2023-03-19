/*
    Appellation: machine <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::{
    tapes::{Tape, Tapes},
    Operator, Program, Turing,
};
use crate::states::{State, Stateful};
use crate::{Scope, Symbolic};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Machine<S: Symbolic = String> {
    program: Program<S>,
    scope: Operator<S>,
}

impl<S: Symbolic> Machine<S> {
    pub fn new(program: Program<S>, scope: Operator<S>) -> Self {
        Self { program, scope }
    }
    pub fn program(&self) -> Program<S> {
        self.program.clone()
    }
    pub fn scope(&self) -> Operator<S> {
        self.scope.clone()
    }
    pub fn tape(&self) -> &Tape<S> {
        self.scope.tape()
    }
}

impl<S: Symbolic> Turing<S> for Machine<S> {
    type Error = Box<dyn std::error::Error>;
    type Scope = Operator<S>;

    fn execute(&mut self) -> Result<&Self, Self::Error> {
        let until = |actor: &Operator<S>| actor.state() == State::Invalid;
        self.execute_until(until)
    }

    fn execute_once(&mut self) -> Result<&Self, Self::Error> {
        let head = self.scope.clone().into();
        let inst = self.program.get(head)?.clone();
        self.scope.update_state(inst.tail().state());
        self.scope.set_symbol(inst.tail().symbol());
        self.scope
            .shift(inst.tail().action(), self.program.default_symbol());
        Ok(self)
    }

    fn execute_until(
        &mut self,
        until: impl Fn(&Self::Scope) -> bool,
    ) -> Result<&Self, Self::Error> {
        while !until(&self.scope) {
            let head = self.scope.clone().into();
            let inst = self.program.get(head)?.clone();
            self.scope.update_state(inst.tail().state());
            self.scope.set_symbol(inst.tail().symbol());
            self.scope
                .shift(inst.tail().action(), self.program.default_symbol());
        }
        Ok(self)
    }

    fn translate(&mut self, tape: Tapes<S>) -> Result<Tape<S>, Self::Error> {
        self.scope = Operator::build(tape);
        self.execute()?;
        Ok(self.scope.tape().clone())
    }
}
