#[cfg(test)]
use contained_core::turing::{
    instructions::{Instruction, Move},
    tapes::{Tape, Tapes},
    Machine, Operator, Program, Turing,
};
use contained_core::{states::State, Extend, Scope};

pub const TEST_ALPHABET: [&str; 3] = ["a", "b", "c"];

#[test]
fn test_machine() {
    let alphabet = vec!["a", "b", "c"];

    let tape = alphabet.clone();
    let scope = Operator::build(Tapes::norm(tape));

    let instructions: Vec<Instruction<&str>> = vec![
        (State::default(), "a", State::default(), "c", Move::Right).into(),
        (State::default(), "b", State::default(), "a", Move::Right).into(),
        (
            State::default(),
            "c",
            State::invalid().into(),
            "a",
            Move::Stay,
        )
            .into(),
    ];

    // Setup the program
    let mut program = Program::new(alphabet, State::invalid());
    // Instruction set; turn ["a", "b", "c"] into ["c", "a", "a"]
    program.extend(instructions).unwrap();

    let mut machine = Machine::new(scope, program);

    assert!(machine.execute().is_ok());
    assert_eq!(machine.tape().clone(), Tape::new(["c", "a", "a"]));
}
