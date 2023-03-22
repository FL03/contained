/*
    Appellation: actors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::actor::*;

mod actor;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::states::{State, Stateful};
    use crate::turing::{
        instructions::{Instruction, Move},
        Program, Tape, Turing,
    };
    use crate::{Extend, Scope, Symbolic};

    pub const TEST_ALPHABET: [&str; 3] = ["a", "b", "c"];

    #[test]
    fn test_actor() {
        let instructions: Vec<Instruction<&str>> = vec![
            (State::default(), "a", State::default(), "c", Move::Right).into(),
            (State::default(), "b", State::default(), "a", Move::Right).into(),
            (State::default(), "c", State::invalid(), "a", Move::Stay).into(),
        ];

        // Setup the program
        let mut program = Program::new(TEST_ALPHABET, State::invalid());
        // Instruction set; turn ["a", "b", "c"] into ["c", "a", "a"]
        program.extend(instructions).unwrap();

        let mut machine = Actor::new(program);
        machine.memory = Tape::norm(["a", "b", "c"]);
        assert!(machine.execute().is_ok());
        assert_eq!(machine.memory.clone(), Tape::norm(["c", "a", "a"]));
    }
}
