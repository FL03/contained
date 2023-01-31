/*
    Appellation: machine <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::{Program, Symbolic};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Machine<S: Symbolic> {
    pub ds: S, // the default symbol
    pub program: Program<S>,
}

impl<S: Symbolic> Machine<S> {
    pub fn new(ds: S, program: Program<S>) -> Result<Self, String> {
        if program.alphabet().contains(&ds) {
            Ok(Self { ds, program })
        } else {
            return Err(format!(
                "The indicated default symbol ({}) is not present in the provided alphabet...",
                ds.to_string()
            ));
        }
    }
    pub fn initial(&self) -> S {
        self.ds.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::turing::{Instruction, Move, Program};
    use crate::States;

    #[test]
    fn test_machine() {
        let inst = Instruction::from((States::Valid, "a", States::Valid, "b", Move::Right));
        let alphabet = vec!["a", "b", "c"];
        let mut program = Program::new(alphabet, 1.into());
        program.insert(inst.clone()).unwrap();

        assert!(Machine::new("a", program.clone()).is_ok());
        assert!(Machine::new("", program.clone()).is_err())
    }
}
