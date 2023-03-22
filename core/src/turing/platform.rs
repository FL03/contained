/*
    Appellation: machine <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::instructions::Instruction;
use super::{Operator, Program, Tape, Turing};
use crate::states::{State, Stateful};
use crate::{Alphabet, Error, Scope, Symbolic};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Machine<S: Symbolic = String> {
    pub program: Program<S>,
    pub scope: Operator<S>,
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

impl<S: Symbolic> Extend<S> for Machine<S> {
    fn extend<T: IntoIterator<Item = S>>(&mut self, iter: T) {
        self.scope.tape.extend(iter)
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
        if let Some(cur) = self.clone().scope.tape().get(self.scope.index()) {
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
    fn default_symbol(&self) -> S {
        self.program.default_symbol()
    }
}

impl<S: Symbolic> Stateful for Machine<S> {
    type State = State;

    fn state(&self) -> Self::State {
        self.scope.state()
    }
    fn update_state(&mut self, state: Self::State) {
        self.scope.update_state(state)
    }
}

impl<S: Symbolic> Turing<S> for Machine<S> {
    type Error = Error;
    type Scope = Operator<S>;

    fn execute(&mut self) -> Result<&Self, Self::Error> {
        let until = |actor: &Operator<S>| actor.state() == State::Invalid;
        self.execute_until(until)
    }

    fn execute_once(&mut self) -> Result<&Self, Self::Error> {
        let head = self.scope.clone().into();
        let inst = self.program.get(head).expect("").clone();
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
            self.execute_once()?;
        }
        Ok(self)
    }

    fn translate(&mut self, tape: Tape<S>) -> Result<Tape<S>, Self::Error> {
        self.scope = Operator::from(tape);
        self.execute()?;
        Ok(self.scope.tape().clone())
    }
}
