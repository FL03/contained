/*
    Appellation: actors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{actor::*, exec::*};

mod actor;
mod exec;

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
