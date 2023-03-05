/*
    Appellation: machine <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::turing::{Operator, Tapes, Turing};
use crate::{Scope, Symbolic};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Machine<S: Symbolic = String> {
    scope: Operator<S>,
}

impl<S: Symbolic> Machine<S> {
    pub fn new(scope: Operator<S>) -> Self {
        Self { scope }
    }
}

impl<S: Symbolic> Turing<S> for Machine<S> {
    type Error = String;
    type Scope = Operator<S>;

    fn driver(&mut self) -> &mut Self::Scope {
        &mut self.scope
    }
    fn update(&mut self, tape: Tapes<S>) {
        self.scope = Operator::build(tape);
    }
}
