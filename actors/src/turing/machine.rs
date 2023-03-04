/*
    Appellation: machine <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::turing::{Operator, Symbolic, Turing};

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
    type Scope = Operator<S>;

    fn driver(&mut self) -> &mut Self::Scope {
        &mut self.scope
    }
    fn update(&mut self, scope: Self::Scope) {
        self.scope = scope;
    }
}
